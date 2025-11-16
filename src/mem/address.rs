use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::constants::VIRTUAL_OFFSET;

/// Enforced power of two alignment struct
#[derive(Clone, Copy)]
pub struct Alignment(usize);

impl Alignment {
    pub fn new(align: usize) -> Self {
        assert!(align > 0 && align.is_power_of_two());
        Self(align)
    }

    pub fn of<T>() -> Self {
        Self::new(align_of::<T>())
    }

    pub fn mask(self) -> usize {
        self.0 - 1
    }
}

impl From<usize> for Alignment {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

/// Helper macro to keep physical / virtual address defs in sync
macro_rules! make_addr_type {
    ($name:ident) => {

        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name(usize);

        impl $name {
            pub fn new(value: usize) -> Self {
                Self(value)
            }

            pub fn as_usize(self) -> usize {
                self.0
            }

            pub fn is_aligned(self, align: Alignment) -> bool {
                self.0 & align.mask() == 0
            }

            pub fn align_down(self, align: Alignment) -> Self {
                Self::new(self.0 & !align.mask())
            }

            pub fn align_up(self, align: Alignment) -> Self {
                let mask = align.mask();
                Self::new((self.0 + mask) & !mask)
            }
        }

        impl Add<usize> for $name {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_add(rhs)
                    .expect("Address offset addition overflowed");
                Self(result)
            }
        }

        impl AddAssign<usize> for $name {
            fn add_assign(&mut self, rhs: usize) {
                *self = *self + rhs;
            }
        }

        impl Sub<usize> for $name {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_sub(rhs)
                    .expect("Address offset subtraction overflowed");
                Self(result)
            }
        }

        impl SubAssign<usize> for $name {
            fn sub_assign(&mut self, rhs: usize) {
                *self = *self - rhs
            }
        }
    };
}

make_addr_type!(PhysicalAddress);


impl PhysicalAddress {

    /// Get a valid virtual address for this physical address (within the 
    /// kernel 512GB mapping). This is not guaranteed to be the only mapping
    pub fn to_virtual(self) -> VirtualAddress {
        VirtualAddress(self.0 + VIRTUAL_OFFSET)
    }
}


make_addr_type!(VirtualAddress);

impl VirtualAddress {

    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as usize)
    }
}
