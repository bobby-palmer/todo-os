use std::{env, fs, path::{Path, PathBuf}};

fn main() {

    // Link RISCV files from asm directory
    let asm_dir = Path::new("src/asm");

    let asm_files: Vec<PathBuf> = fs::read_dir(asm_dir).unwrap()
        .filter_map(|maybe_file| {
            let path = maybe_file.ok()?.path();
            if path.extension()?.to_str() == Some("S") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    cc::Build::new()
        .files(asm_files)
        .flag("-march=rv64gc")  // Specify Architecture
        .flag("-mabi=lp64d")   // Specify ABI to match Rust's FPU usage
        .compile("asm");

    println!("cargo:rerun-if-changed={}", asm_dir.display());

    // Set linker script
    let ld_name = "kernel.ld";
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let link_path = manifest_dir.join(ld_name);
    let linker_script_path = link_path.to_str().unwrap();

    println!("cargo:rustc-link-arg=-T{}", linker_script_path);
    println!("cargo:rerun-if-changed={}", ld_name);
}
