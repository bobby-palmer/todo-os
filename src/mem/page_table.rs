//! Sv39 page table implementation, with minimal abstractions

use core::ops::Add;

/// Permissions for mapping
#[derive(Clone, Copy)]
pub enum Flag {
    Valid,
    Read,
    Write,
    Execute,
    User,
    Global,
    Accessed,
    Dirty,
}

impl Flag {
    fn bit(&self) -> u64 {
        let idx = match self {
            Flag::Valid => 0,
            Flag::Read => 1,
            Flag::Write => 2,
            Flag::Execute => 3,
            Flag::User => 4,
            Flag::Global => 5,
            Flag::Accessed => 6,
            Flag::Dirty => 7,
        };

        1 << idx
    }
}

impl Add for Flag {
    type Output = FlagSet;

    fn add(self, rhs: Self) -> Self::Output {
        FlagSet(0) + self + rhs
    }
}

/// A collection of permission flags as a bitset
#[derive(Clone, Copy)]
pub struct FlagSet(u64);

impl FlagSet {
    fn contains(&self, flag: Flag) -> bool {
        self.0 & flag.bit() != 0
    }
}

impl Add for FlagSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Add<Flag> for FlagSet {
    type Output = Self;

    fn add(self, rhs: Flag) -> Self::Output {
        Self(self.0 | rhs.bit())
    }
}

impl From<Flag> for FlagSet {
    fn from(value: Flag) -> Self {
        Self(value.bit())
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Pte(u64);

impl Pte {
    fn new(ppn: u64, flags: FlagSet) -> Self {
        Self((ppn << 10) | flags.0)
    }

    /// Return physical page number if this mapping is valid
    fn ppn(&self) -> Option<u64> {
        if self.flags().contains(Flag::Valid) {
            Some(self.0 >> 10)
        } else {
            None
        }
    }

    fn flags(&self) -> FlagSet {
        FlagSet(self.0 & 0xFF)
    }
}

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {
    pub fn map_page(&mut self) -> Result<(), &'static str> {
        todo!()
    }

    pub fn unmap(&mut self) -> Result<(), &'static str> {
        todo!()
    }

    pub fn translate(&self, vaddr: usize) -> Option<usize> {
        todo!()
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        todo!()
    }
}
