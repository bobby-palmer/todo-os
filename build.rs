use std::env;
use std::fs;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Watch the entire asm directory for changes
    println!("cargo:rerun-if-changed=src/asm");

    // Only compile assembly for the RISC-V target
    if target.contains("riscv64") {
        let mut build = cc::Build::new();
        build
            .flag("-march=rv64gc")
            .flag("-mabi=lp64d");

        // Add all .S and .s files from the asm directory
        if let Ok(entries) = fs::read_dir("src/asm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "S" || ext == "s" {
                        build.file(&path);
                    }
                }
            }
        }

        build.compile("asm");
    }
}
