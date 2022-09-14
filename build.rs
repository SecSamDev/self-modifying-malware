use std::io;
#[cfg(windows)] use winres::WindowsResource;
#[cfg(windows)] use static_vcruntime;fn main() -> io::Result<()> {
    #[cfg(windows)] {
        static_vcruntime::metabuild();
        WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
    }
    Ok(())
}