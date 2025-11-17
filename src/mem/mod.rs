use fdt::Fdt;

/// One time initialization for the boot hart to initialize free ram
pub fn init(_fdt: &Fdt) {

}

pub mod page_table;
pub mod common;
