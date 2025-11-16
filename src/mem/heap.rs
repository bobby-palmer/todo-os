use core::{alloc::{GlobalAlloc, Layout}, cmp::max, ptr::{self, NonNull}};

use spin::Mutex;

use crate::mem::{address::Page, pfa};

#[global_allocator]
static HEAP: Heap = Heap::new();

struct Heap {
    caches: [Mutex<Cache>; 8]
}

impl Heap {
    const fn new() -> Self {
        let caches = [
            Mutex::new(Cache::new(Cache::MIN_ORDER + 0)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 1)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 2)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 3)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 4)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 5)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 6)),
            Mutex::new(Cache::new(Cache::MIN_ORDER + 7)),
        ];

        Self {
            caches
        }
    }

    fn get_order(layout: Layout) -> usize {
        let bytes = layout.size();
        let align = layout.align();
        let min_b = max(bytes, align);
        let min_o = min_b.ilog2() +
            if min_b.is_power_of_two() { 0 } else { 1 };

        max(min_o as usize, Cache::MIN_ORDER)
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let cache_order = Self::get_order(layout);
        let cache_index = cache_order - Cache::MIN_ORDER;

        if cache_index < self.caches.len() {
            let result = self.caches[cache_index].lock().alloc();

            match result {
                Some(ptr) => ptr.as_ptr(),
                None => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let cache_order = Self::get_order(layout);
        let cache_index = cache_order - Cache::MIN_ORDER;
        if cache_index < self.caches.len() {
            self.caches[cache_index].lock().free(NonNull::new(ptr)
                .unwrap());
        } else {
            todo!()
        }
    }
}

struct Cache {
    order: usize,
    slabs: Option<NonNull<Slab>>,
}

unsafe impl Send for Cache {}

impl Cache {
    const MIN_ORDER: usize = 3;

    const fn new(order: usize) -> Self {
        Self {
            order,
            slabs: None,
        }
    }

    fn alloc(&mut self) -> Option<NonNull<u8>> {
        unsafe {
            if self.slabs.is_none() {
                self.prepend(Slab::new(1 << self.order)?);
            }

            let mut slab = self.slabs?;
            let result = slab.as_mut().alloc();

            if slab.as_ref().is_fully_taken() {
                self.pop();   
            }

            result
        }
    }

    fn free(&mut self, ptr: NonNull<u8>) {
        unsafe {
            let mut slab = Slab::get_owner(ptr);

            if slab.as_ref().is_fully_taken() {
                self.prepend(slab);
            }

            slab.as_mut().free(ptr);
        }
    }

    fn prepend(&mut self, mut slab: NonNull<Slab>) {
        unsafe {
            slab.as_mut().next = self.slabs;
            self.slabs = Some(slab);
        }
    }

    fn pop(&mut self) {
        unsafe {
            if let Some(mut slab) = self.slabs {
                self.slabs = slab.as_ref().next;
                slab.as_mut().next = None;
            }
        }
    }
}

struct Slab {
    next: Option<NonNull<Self>>,
    slots: Slot,
    used: usize,
}


impl Slab {
    fn new(alloc_size: usize) -> Option<NonNull<Self>> {
        unsafe {
            let page = pfa::alloc()?;
            let mut me = page.cast::<Slab>();

            me.write(Slab {
                next: None,
                slots: Slot(None),
                used: 0
            });

            let mut slot = me.add(1).cast::<u8>();

            while slot.add(alloc_size) <= page.add(1).cast() {
                me.as_mut().slots.prepend(slot.cast());
                slot = slot.add(alloc_size);
            }

            Some(me)
        }
    }

    fn get_owner(ptr: NonNull<u8>) -> NonNull<Self> {
        let offset = ptr.align_offset(size_of::<Page>());

        unsafe { 
            ptr
            .add(offset)
            .sub(size_of::<Page>())
            .cast()
        }
    }

    fn alloc(&mut self) -> Option<NonNull<u8>> {
        let slot = self.slots.pop()?;
        self.used += 1;
        Some(slot.cast())
    }

    fn free(&mut self, ptr: NonNull<u8>) {
        self.slots.prepend(ptr.cast());
        self.used -= 1;
    }

    fn is_fully_taken(&self) -> bool {
        self.slots.0.is_none()
    }

}

#[derive(Clone, Copy)]
struct Slot(Option<NonNull<Self>>);

impl Slot {
    fn prepend(&mut self, slot: NonNull<Slot>) {
        unsafe { slot.write(*self); }
        self.0 = Some(slot);
    }

    fn pop(&mut self) -> Option<NonNull<Slot>> {
        let node = self.0?;
        unsafe {*self = node.read();}
        Some(node)
    }
}
