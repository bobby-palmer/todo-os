use core::fmt;
use core::{fmt::{Debug, LowerHex, UpperHex}, ops::{Add, Sub}};

/// Base address traits, types must implement all of these to use the impl 
/// address macro
pub trait AddressMarker:
    Sized + Copy + Clone + Eq + PartialEq 
    + Ord + PartialOrd + Debug + Default {}

pub trait Address: 
AddressMarker
+ Into<u64>
+ From<u64>
+ Add<usize, Output = Self>
+ Sub<usize, Output = Self>
+ Sub<Self, Output = usize>
+ LowerHex
+ UpperHex
{

    fn is_aligned(self, align: usize) -> bool {
        assert!(align > 0 && align.is_power_of_two());
        let addr: u64 = self.into();
        (addr & (align as u64 - 1)) == 0
    }

    fn align_down(self, align: usize) -> Self {
        assert!(align > 0 && align.is_power_of_two());
        let addr: u64 = self.into();
        let mask = align as u64 - 1;
        let aligned_addr = addr & !mask;
        Self::from(aligned_addr)
    }

    fn align_up(self, align: usize) -> Self {
        assert!(align > 0 && align.is_power_of_two());
        let addr: u64 = self.into();
        let mask = align as u64 - 1;
        let aligned_addr = (addr.wrapping_add(mask)) & !mask;
        Self::from(aligned_addr)
    }
}

/// Implement non AddressMarker traits for tuple struct address
macro_rules! impl_address {
    ($type:ident) => {
        impl AddressMarker for $type {}        

        impl From<u64> for $type {
            fn from(addr: u64) -> Self {
                Self(addr)
            }
        }
        
        impl Into<u64> for $type {
            fn into(self) -> u64 {
                self.0
            }
        }

        impl Add<usize> for $type {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_add(rhs as u64)
                    .expect("Address addition overflowed!");
                Self(result)
            }
        }

        impl Sub<usize> for $type {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                let result = self.0.checked_sub(rhs as u64)
                    .expect("Address subtraction overflowed");
                Self(result)
            }
        }

        impl Sub<Self> for $type {
            type Output = usize;
            fn sub(self, rhs: Self) -> Self::Output {
                let result = self.0.checked_sub(rhs.0)
                    .expect("Address difference overflowed");
                result as usize
            }
        }

        impl LowerHex for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        impl UpperHex for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#X}", self.0)
            }
        }

        impl Address for $type {}
    };
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
pub struct PhyicalAddress(u64);
impl_address!(PhyicalAddress);

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
pub struct VirtualAddress(u64);
impl_address!(VirtualAddress);
