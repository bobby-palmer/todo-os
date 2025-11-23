use fdt::Fdt;
use page_table_multiarch::PagingHandler;
use memory_addr::{MemoryAddr, PhysAddr, VirtAddr};
use spin::Mutex;
use talc::{OomHandler, Talc, Talck};

pub const PAGE_SIZE: usize = 0x1000;
pub const PHYSICAL_RAM_START: usize =        0x80000000;
pub const VIRTUAL_RAM_START: usize = 0xffffffc000000000;

/// One time boot function
pub fn init(fdt: &Fdt) {
    let ram = fdt.memory().regions().next().unwrap();

    let ram_start = PhysAddr::from_usize(ram.starting_address as usize);
    let ram_end = ram_start + ram.size.unwrap();

}

/// Return virtual address in the kernel linear map
fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    let paddr = paddr.as_usize();
    let vaddr = paddr.sub(PHYSICAL_RAM_START).add(VIRTUAL_RAM_START);
    VirtAddr::from_usize(vaddr)
}

/// Intrusive Physical page linked list
struct PhysPageList {
    length: usize,
    head: Option<PhysAddr>,
}

impl PhysPageList {

    /// Construct empty page list
    const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    /// Number of pages in this list
    fn len(&self) -> usize {
        self.length
    }

    fn prepend(&mut self, page: PhysAddr) {
        unsafe {
            let virt_addr = phys_to_virt(page);
            let virt_ptr: *mut Option<PhysAddr> = virt_addr.as_mut_ptr_of();
            virt_ptr.write(self.head);
            self.head = Some(page);
            self.length += 1;
        }
    }

    fn pop(&mut self) -> Option<PhysAddr> {
        unsafe {
            let page = self.head?;
            let virt_addr = phys_to_virt(page);
            let virt_pointer: *const Option<PhysAddr> = virt_addr.as_ptr_of();
            self.head = virt_pointer.read();
            self.length -= 1;

            Some(page)
        }
    }
}

/// List of free physical pages
static PHYSICAL_PAGE_FREE_LIST: Mutex<PhysPageList> = 
    Mutex::new(PhysPageList::new());

/// Kernel virtual memory manager
struct Vmm {
    ram_start: VirtAddr,
    ram_end: VirtAddr,
    heap_start: VirtAddr,
    heap_end: VirtAddr,
}

impl Vmm {
    fn new(ram_start: VirtAddr, ram_end: VirtAddr) -> Self {
        todo!()
    }

    /// Grow kernel heap by one page
    fn extend_heap(&mut self) {

    }
}

/// Page table trait to power library functions
pub struct PagingHandlerImpl;

impl PagingHandler for PagingHandlerImpl {
    fn alloc_frame() -> Option<PhysAddr> {
        PHYSICAL_PAGE_FREE_LIST.lock().pop()
    }

    fn dealloc_frame(paddr: PhysAddr) {
        PHYSICAL_PAGE_FREE_LIST.lock().prepend(paddr);
    }

    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
        phys_to_virt(paddr)
    }
}

/// Handle heap expansion for kernel heap
struct HeapOomHandler;

impl OomHandler for HeapOomHandler {
    fn handle_oom(_talc: &mut Talc<Self>, _layout: core::alloc::Layout) -> Result<(), ()> {
        todo!()
    }
}

#[global_allocator]
static ALLOCATOR: Talck<Mutex<()>, HeapOomHandler> = Talc::new(HeapOomHandler).lock();
