use core::ptr::NonNull;

/// Start of the kernels persistent 512GB linear map of ram
pub const VIRTUAL_OFFSET: usize = 0xffff800000000000; 

/// Rust repr for a single 4KB Page
#[repr(align(0x1000), C)]
pub struct Page([u8; 0x1000]);

/// Helper macro to keep address types in sync
macro_rules! make_addr_type {
    ($name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(usize);

        impl $name {
            pub fn new(addr: usize) -> Self {
                Self(addr)
            }

            pub fn null() -> Self {
                Self(0)
            }
        }
    };
}

make_addr_type!(PhysicalAddress);

impl PhysicalAddress {}

impl From<PhysicalPageNumber> for PhysicalAddress {
    fn from(value: PhysicalPageNumber) -> Self {
        Self(value.0 as usize * size_of::<Page>())
    }
}

pub struct PhysicalPageNumber(pub u64);

impl PhysicalPageNumber {}

make_addr_type!(VirtualAddress);

impl VirtualAddress {

    /// Try to cast address as pointer and panic if misaligned
    pub fn as_ptr<T>(&self) -> Option<NonNull<T>> {
        let ptr = NonNull::new(self.0 as *mut T)?;

        if ptr.is_aligned() {
            Some(ptr)
        } else {
            panic!("Try to access virtual address as misaligned pointer");
        }
    }
}

impl From<PhysicalAddress> for VirtualAddress {
    fn from(value: PhysicalAddress) -> Self {
        Self(value.0 + VIRTUAL_OFFSET)
    }
}

impl From<VirtualPageNumber> for VirtualAddress {
    fn from(value: VirtualPageNumber) -> Self {
        Self(value.0 as usize * size_of::<Page>())
    }
}

pub struct VirtualPageNumber(pub u64);

impl VirtualPageNumber { }
