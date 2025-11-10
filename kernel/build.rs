use std::env;
use std::path::PathBuf;

fn main() {

    cc::Build::new()
        .file("src/asm/boot.S") 
        .flag("-march=rv64gc")  // Specify Architecture
        .flag("-mabi=lp64d")   // Specify ABI to match Rust's FPU usage
        .compile("asm_entry");

    // Set linker script
    // Get the path to the directory containing this build script (the kernel crate root)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Assuming link.ld is in the kernel/ directory or the workspace root.
    // If it's in the workspace root, you'll need to go up one level (manifest_dir.parent().unwrap())
    let link_path = manifest_dir.join("link.ld"); 

    // Convert the absolute path to a string
    let linker_script_path = link_path.to_str().unwrap();

    // Pass the ABSOLUTE path to the linker
    println!("cargo:rustc-link-arg=-T{}", linker_script_path);

    // File change detection
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/asm/boot.S");
    println!("cargo:rerun-if-changed=link.ld");
}
