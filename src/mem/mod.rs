pub mod page_table;
pub mod common;
mod page_list;

use core::ptr::NonNull;

use crate::{mem::common::{Page, PAGE_SIZE, PHYSICAL_RAM_START, VIRTUAL_RAM_START}, println};
use fdt::Fdt;
use spin::Mutex;
use page_list::PageList;

unsafe extern "C" {
    static mut _kend: u8;
}

/// One time initialization for the boot hart to initialize free ram
pub fn init(fdt: &Fdt) {
    unsafe {
        let ram = fdt.memory().regions().next().unwrap();
        let ram_start = ram.starting_address.sub(PHYSICAL_RAM_START)
            .wrapping_add(VIRTUAL_RAM_START);

        let ram_end = ram_start.add(ram.size.unwrap());

        println!("init ram with start: {ram_start:?}, end: {ram_end:?}");

        // 1) TODO ensure all ram is mapped

        // 2) Add all free ram to the list
        let is_reserved = |_page: *const Page| {
            // TODO skip the fdt blob
            if _page < (&raw const _kend).cast() {
                true
            } else {
                false
            }
        };

        let offset = ram_start.align_offset(PAGE_SIZE);
        let mut current_page = ram_start.add(offset).cast::<Page>();

        while current_page.add(1) < ram_end.cast() {
            if !is_reserved(current_page) {
                FREE_PAGES.lock().prepend(
                    NonNull::new_unchecked(current_page as *mut Page));
            }

            current_page = current_page.add(1);
        }

        println!("Free page list has {} pages", FREE_PAGES.lock().len());

        // 3) Setup kernel mapping spaces TODO
    }
}

/// List of free page buffers to use for single page allocations
static FREE_PAGES: Mutex<PageList> = Mutex::new(PageList::new());
