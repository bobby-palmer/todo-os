const VMA_OFFSET: usize = 0xffff800000000000;
const PAGE_SIZE: usize = 0x1000;

struct PhysAddr(usize);

struct BuddyAllocator {

}

impl BuddyAllocator {
    
    fn new(_start_free_ram: PhysAddr, _end_free_ram: PhysAddr) -> Self {
        todo!()
    }

    fn alloc(&mut self) {
        todo!()
    }

    fn alloc_page(&mut self) {

    }

    fn free(&mut self) {

    }

    fn free_page(&mut self) {

    }

}
