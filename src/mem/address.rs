use core::{fmt::{LowerHex, UpperHex}, ops::{Add, AddAssign, Sub, SubAssign}, ptr::NonNull};

use crate::constants::VIRTUAL_OFFSET;

#[repr(C)]
#[repr(align(0x1000))]
pub struct Page {
    bytes: [u8; 0x1000]
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Alignment(usize);

impl From<usize> for Alignment {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl Alignment {

    pub fn new(value: usize) -> Self {
        if value == 0 || !value.is_power_of_two() {
            panic!("Invalid alignment");
        }

        Self(value)
    }

    pub fn of<T>() -> Self {
        Self::new(align_of::<T>())
    }

    pub fn mask(self) -> usize {
        self.0 - 1
    }
}

macro_rules! make_addr_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name(usize);

        impl $name {
            pub fn new(value: usize) -> Self {
                Self(value)
            }

            pub fn null() -> Self {
                Self(0)
            }

            pub fn is_null(self) -> bool {
                self.0 == 0
            }

            pub fn as_int(self) -> usize {
                self.0
            }

            pub fn is_aligned(self, align: Alignment) -> bool {
                self.0 & align.mask() == 0
            }

            pub fn align_down(self, align: Alignment) -> Self {
                Self(self.0 & !align.mask())
            }

            pub fn align_up(self, align: Alignment) -> Self {
                let mask = align.mask();
                Self((self.0 + mask) & !mask)
            }

        }

        impl LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:x}", self.0)
            }
        }

        impl UpperHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:X}", self.0)
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
                let result = *self + rhs;
                *self = result;
            }
        }

        impl Sub<usize> for $name {
            type Output = Self;

            fn sub(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_sub(rhs)
                    .expect("Address offset substraction overflowed");
                Self(result)
            }
        }

        impl SubAssign<usize> for $name {
            fn sub_assign(&mut self, rhs: usize) {
                let result = *self - rhs;
                *self = result;
            }
        }
    };
}

make_addr_type!(PhysicalAddress);

impl PhysicalAddress {

    /// Return the virtual mapping in the kernel persistent map at start of 
    /// address space. Not guaranteed to be the only mapping!
    pub fn to_virt(self) -> VirtualAddress {
        VirtualAddress(self.0 + VIRTUAL_OFFSET)
    }
}

make_addr_type!(VirtualAddress);

impl VirtualAddress {
    pub fn from_ptr<T>(ptr: Option<NonNull<T>>) -> Self {
        match ptr {
            Some(ptr) => Self::new(ptr.addr().get()),
            None => Self::null()
        }
    }

    pub fn as_ptr<T>(self) -> Option<NonNull<T>> {
        if !self.is_aligned(Alignment::of::<T>()) {
            panic!("Invalid alignment for ptr")
        }

        NonNull::new(self.0 as *mut T)
    }

}
