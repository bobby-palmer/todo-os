use core::ptr::NonNull;

use crate::mem::{address::Page, pfa};


struct Cache {
    order: usize,
    slabs: Option<NonNull<Slab>>,
}

impl Cache {
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
                
            }

            result
        }
    }

    fn free(&mut self, ptr: NonNull<u8>) {
        todo!()
    }

    fn prepend(&mut self, slab: NonNull<Slab>) {
        todo!()
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
