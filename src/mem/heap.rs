//! Global heap allocator that can be accessed via core::alloc to allocate in 
//! kernel space

use core::{alloc::{GlobalAlloc, Layout}, cmp::max, ptr::NonNull};

use spin::Mutex;

use crate::mem::{page_list, Page};

#[global_allocator]
static GLOBAL_HEAP: KernelHeap = KernelHeap::new();

/// General purpose global allocator that dispatches between slab and page 
/// allocation
struct KernelHeap {
    caches: [Mutex<Cache>; 8],
}

impl KernelHeap {
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
            caches,
        }
    }

    /// get the potential "order" slab to allocate from
    fn get_cache_order(layout: Layout) -> u32 {
        let cache_bytes = max(layout.size(), layout.align());
        let cache_order = cache_bytes.ilog2() +
            if cache_bytes.is_power_of_two() { 0 } else { 1 };

        max(cache_order, Cache::MIN_ORDER)
    }
}

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let order = Self::get_cache_order(layout);
        let index = (order - Cache::MIN_ORDER) as usize;

        if index < self.caches.len() {
            todo!()
        } else {
            todo!() // Large allocation
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let order = Self::get_cache_order(layout);
        let index = (order - Cache::MIN_ORDER) as usize;

        if index < self.caches.len() {
            todo!()
        } else {
            todo!() // Large allocation
        }
    }
}

/// A collection of slabs
struct Cache {
    head: Option<NonNull<Slab>>,
    order: u32,
}

impl Cache {
    // must have atleast enough for pointer in slots
    const MIN_ORDER: u32 = 3;

    const fn new(order: u32) -> Self {
        Self {
            head: None,
            order,
        }
    }

    unsafe fn alloc(&mut self) -> Option<NonNull<u8>> {
        unsafe {
            let mut slab = self.head?;
            let slot = slab.as_mut().alloc().unwrap();

            if slab.as_ref().slots.0.is_none() {
                self.head = slab.as_ref().next;
            }

            Some(slot)
        }
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>) {
        unsafe {
            let mut slab = Slab::get_slab_for_ptr(ptr);

            if slab.as_ref().is_empty() {
                slab.as_mut().next = self.head;
                self.head = Some(slab);
            }

            slab.as_mut().dealloc(ptr);

            if slab.as_ref().used_slots == 0 {
                page_list::free(slab.cast());
            }
        }
    }
}

/// A one page buffer that hands out buffers of "alloc_size" size
struct Slab {
    used_slots: usize,
    next: Option<NonNull<Self>>,
    slots: SlabLink,
}

impl Slab {
    unsafe fn get_slab_for_ptr(ptr: NonNull<u8>) -> NonNull<Self> {
        let offset = ptr.align_offset(size_of::<Page>());
        unsafe {
            ptr.add(offset).sub(size_of::<Page>()).cast()
        }
    }

    unsafe fn new(alloc_size: usize) -> Option<NonNull<Self>> {
        let page = page_list::alloc()?;

        let mut me = page.cast::<Slab>();

        unsafe {
            me.write(Self {
                used_slots: 0,
                next: None,
                slots: SlabLink(None),
            });

            let after_header = me.add(1).cast::<u8>();
            let mut slot_ptr = after_header.add(after_header.align_offset(alloc_size));

            while slot_ptr.add(alloc_size) < page.add(1).cast() {
                let link = slot_ptr.cast::<SlabLink>();
                link.write(me.as_ref().slots);
                me.as_mut().slots.0 = Some(link);
                slot_ptr = slot_ptr.add(alloc_size);
            }

        }

        Some(me)
    }

    unsafe fn alloc(&mut self) -> Option<NonNull<u8>> {
        let slot = self.slots.0?;

        unsafe { self.slots = slot.read(); }
        self.used_slots += 1;

        Some(slot.cast())
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>) {
        let node = ptr.cast::<SlabLink>();
        unsafe { node.write(self.slots); }
        self.slots.0 = Some(node);
        self.used_slots -= 1;
    }

    /// true -> no more empty slots
    fn is_empty(&self) -> bool {
        self.slots.0.is_none()
    }
}

#[derive(Clone, Copy)]
struct SlabLink(Option<NonNull<Self>>);
