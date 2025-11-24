//! Addr types (Physical and Virtual)

use core::{ops::Add, ptr::NonNull};

use crate::memory::{PHYSICAL_RAM_START, VIRTUAL_RAM_START};

macro_rules! make_addr_type {
    ($name:ident) => {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct $name(usize);

        impl $name {
            pub fn of_usize(addr: usize) -> Self {
                Self(addr)
            }

            pub fn as_usize(&self) -> usize {
                self.0
            }
        }

        impl Add<usize> for $name {
            type Output = Self;

            fn add(self, rhs: usize) -> Self::Output {
                Self(self.0 + rhs)
            }
        }
    };
}

make_addr_type!(PhysicalAddr);


impl PhysicalAddr {

    /// Return a valid mapping to this physical page in the kernel linear map
    pub fn get_virtual(&self) -> VirtualAddr {
        VirtualAddr(self.0 - PHYSICAL_RAM_START + VIRTUAL_RAM_START)
    }
}

make_addr_type!(VirtualAddr);

impl VirtualAddr {

    pub fn as_ptr<T>(&self) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(self.0 as *mut T) }
    }
}
