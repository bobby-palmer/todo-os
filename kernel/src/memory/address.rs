use core::ops::{Add, AddAssign, Sub};

use crate::memory::KERNEL_SPACE_START;

/// Represents a memory alignment guaranteed to be a power of two
#[derive(Debug, Clone, Copy)]
pub struct Alignment(u8);

impl Alignment {
    pub fn of<T>() -> Self {
        align_of::<T>().into()
    }

    pub fn mask(self) -> usize {
        let num: usize = self.into();
        return num - 1;
    }
}

impl From<usize> for Alignment {
    fn from(value: usize) -> Self {
        assert!(value > 0 && value.is_power_of_two());
        Self(value.ilog2() as u8)
    }
}

impl Into<usize> for Alignment {
    fn into(self) -> usize {
        (1 as usize) << (self.0 as u32)
    }
}

macro_rules! make_address_type {
    ($name:ident) => {

        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name(usize);

        impl $name {
            pub fn align_down(self, align: Alignment) -> Self {
                Self(self.0 & !align.mask())
            }

            pub fn align_up(self, align: Alignment) -> Self {
                let mask = align.mask();
                Self((self.0 + mask) & !mask)
            }

            pub fn is_aligned(self, align: Alignment) -> bool {
                self.0 & align.mask() == 0
            }

            pub fn null() -> Self {
                Self(0)
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl Into<usize> for $name {
            fn into(self) -> usize {
                self.0
            }
        }

        impl Add<usize> for $name {
            type Output = Self;

            fn add(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_add(rhs)
                    .expect("Address offset addition overflowed!");
                Self(result)
            }
        }

        impl AddAssign<usize> for $name {
            fn add_assign(&mut self, rhs: usize) {
                let rhs = *self + rhs;
                *self = rhs;
            }
        }

        impl Sub<usize> for $name {
            type Output = Self;

            fn sub(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_sub(rhs)
                    .expect("Address offset subtraction overflowed!");
                Self(result)
            }
        }
    };
}

make_address_type!(PhysicalAddress);

impl Into<VirtualAddress> for PhysicalAddress {
    /// Return a virtual address that is mapped to this physical address 
    /// within the 512GB kernel mapping at the start of kernel address space.
    fn into(self) -> VirtualAddress {
        let result = self.0.checked_add(KERNEL_SPACE_START)
            .expect("VMA_OFFSET addition overflowed physical address");
        VirtualAddress(result)
    }
}

make_address_type!(VirtualAddress);

impl VirtualAddress {
    pub fn as_ptr<T>(self) -> *const T {
        assert!(self.is_aligned(Alignment::of::<T>()));
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        assert!(self.is_aligned(Alignment::of::<T>()));
        self.0 as *mut T
    }
}
