# self-modifying-malware
Self-modifying malware in Rust

```
PS Z:\self-modifying-malware> cargo build --release
   Compiling serde v1.0.144
   Compiling crc32fast v1.3.2
   Compiling memchr v2.5.0
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling memmap2 v0.5.7
   Compiling toml v0.5.9
   Compiling winres v0.1.12
   Compiling self-modifying-malware v0.1.0 (Z:\self-modifying-malware)
    Finished release [optimized] target(s) in 1m 30s
PS Z:\self-modifying-malware> .\target\release\self-modifying-malware.exe
Previous run count: 17413829644165553245
PS Z:\self-modifying-malware> .\target\release\self-modifying-malware.exe
Previous run count: 17413829644165553246
PS Z:\self-modifying-malware> .\target\release\self-modifying-malware.exe
Previous run count: 17413829644165553247
PS Z:\self-modifying-malware> .\target\release\self-modifying-malware.exe
Previous run count: 17413829644165553248
```