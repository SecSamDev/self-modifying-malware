use memmap2::{MmapOptions, MmapMut};
use object::{File, Object, ObjectSection};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use anyhow::{Result, anyhow};

const ICON_COUNTER_POSITION : usize = 256;

fn get_section(file: &File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        match section.name() {
            Ok(n) if n == name => {
                return section.file_range();
            }
            _ => {}
        }
    }
    None
}

fn localize_counter_section(buf : &MmapMut, section_size : usize, section_base : usize) -> Result<usize> {
    let base_buff = &buf[section_base..(section_base +section_size)];
    // https://lief-project.github.io/doc/stable/tutorials/07_pe_resource.html
    let resource_image_directory = &base_buff[..16];
    let named_entries = u16::from_le_bytes(resource_image_directory[12..14].try_into()?);
    let id_entries = u16::from_le_bytes(resource_image_directory[14..].try_into()?);
    let resource_entries = named_entries + id_entries;

    let mut actual_offset = 16;
    let mut section_contents = Vec::new();

    let mut max_offset = 0;
    for resouce in 0..resource_entries {
        // Type entry
        let resource_entry_buffer = &base_buff[actual_offset..actual_offset + 8];
        let resource_type = u16::from_le_bytes(resource_entry_buffer[0..2].try_into()?);
        let _name_is_string =  u16::from_le_bytes(resource_entry_buffer[2..4].try_into()?);
        let name_dir_offset = u16::from_le_bytes(resource_entry_buffer[4..6].try_into()?) as usize;
        
        // Name Directory entry
        let language_buffer = &base_buff[name_dir_offset..name_dir_offset+24];
        let language_offset = u16::from_le_bytes(language_buffer[20..22].try_into()?) as usize;
        
        // Language Directory entry
        let language_buffer = &base_buff[language_offset..language_offset+24];
        let data_entry_offset = u16::from_le_bytes(language_buffer[20..22].try_into()?) as usize;
        
        if data_entry_offset > max_offset {
            max_offset = data_entry_offset;
        }
        // Data entry
        let data_entry_buffer = &base_buff[data_entry_offset..data_entry_offset+16];
        let file_entry_offset = u32::from_le_bytes(data_entry_buffer[0..4].try_into()?) as usize;
        let file_entry_size = u32::from_le_bytes(data_entry_buffer[4..8].try_into()?) as usize;
        section_contents.push((resouce, file_entry_offset, file_entry_size, resource_type));
        actual_offset += 8;
    }
    section_contents.sort_by(|a,b|  (a.1).cmp(&b.1));
    let file_reposition = section_contents.get(0).unwrap().1 - (max_offset + 16 + 8);
    for section in &section_contents {
        if section.3 == 3 { //RT_ICON
            return Ok(section.1 - file_reposition + ICON_COUNTER_POSITION)
        }
    }
    return Err(anyhow!("Icon is missing!"))
}


fn run_count() -> Result<u64> {
    let exe = env::current_exe()?;
    let file = OpenOptions::new().read(true).write(true).open(&exe)?;
    let buf = unsafe { MmapOptions::new().map_mut(&file)? };
    let file = File::parse(&*buf)?;
    
    match get_section(&file, ".rsrc") {
        Some(range) => {
            let section_size = range.1 as usize;
            let section_base = range.0 as usize;
            let base_buff = &buf[section_base..(section_base +section_size)];
            let counter_position = localize_counter_section(&buf,section_size, section_base )?;
            let counter = &base_buff[counter_position..counter_position + 8];
            let counter = u64::from_le_bytes(counter.try_into()?);
            return Ok(counter)
        },
        None => Err(anyhow!("Resource section is missing!"))
    }
}


fn edit_run_count(exe : &PathBuf, counter : u64) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(&exe)?;
    let mut buf = unsafe { MmapOptions::new().map_mut(&file)? };
    let file = File::parse(&*buf)?;
    
    match get_section(&file, ".rsrc") {
        Some(range) => {
            let section_size = range.1 as usize;
            let section_base = range.0 as usize;
            let counter_position = localize_counter_section(&buf,section_size, section_base )?;
            buf[(section_base + counter_position)..(section_base + counter_position + 8)].copy_from_slice(&(counter).to_ne_bytes());
            return Ok(())
        },
        None => Err(anyhow!("Resource section is missing!"))
    }
    
}

fn main() -> Result<(), Box<dyn Error>> {
    let run_count = run_count()?;
    println!("Previous run count: {}", run_count);
    let exe = env::current_exe()?;
    let tmp = exe.with_extension("tmp");
    fs::copy(&exe, &tmp)?;
    edit_run_count(&tmp, run_count + 1)?;
    let perms = fs::metadata(&exe)?.permissions();
    fs::set_permissions(&tmp, perms)?;
    fs::rename(&tmp, &exe)?;

    Ok(())
}