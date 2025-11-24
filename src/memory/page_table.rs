//! Sv39 page table library

use core::ops::Add;

/// Mapping permission flags
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Flags {
    Valid = 0,
    Read = 1,
    Write = 2,
    Execute = 3,
    User = 4,
    Global = 5,
    Accessed = 6,
    Dirty = 7,
}

impl Flags {
    fn bit(&self) -> u64 {
        1 << (*self as u8)
    }
}

/// A bit set of Flags
#[derive(Copy, Clone)]
pub struct FlagSet(u64);

impl FlagSet {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn from_flag(flag: Flags) -> Self {
        Self(flag.bit())
    }

    pub fn add_flag(&self, flag: Flags) -> Self {
        Self(self.0 | flag.bit())
    }

    fn contains(&self, flag: Flags) -> bool {
        self.0 & flag.bit() != 0
    }
}

impl From<Flags> for FlagSet {
    fn from(value: Flags) -> Self {
        Self::from_flag(value)
    }
}

impl Add<Flags> for FlagSet {
    type Output = Self;

    fn add(self, rhs: Flags) -> Self::Output {
        self.add_flag(rhs)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Pte(u64);

impl Pte {
    fn new(ppn: u64, flags: FlagSet) -> Self {
        Self((ppn << 10) | flags.0)
    }

    fn flags(&self) -> FlagSet {
        FlagSet(self.0 & 0xFF)
    }

    /// Return physical page mapping if its valid
    fn ppn(&self) -> Option<u64> {
        if self.flags().contains(Flags::Valid) {
            Some(self.0 >> 10)
        } else {
            None
        }
    }
}

/// Sv39 Level
pub enum MapLevel {
    Huge,
    Big,
    Page
}

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {

    pub fn map(
        &mut self, 
        vpn: u64, 
        ppn: u64, 
        level: MapLevel,
        flags: FlagSet) -> Result<(), ()> 
    {
        todo!()
    }

    pub fn unmap(&mut self, vpn: u64) -> Result<(), ()> {
        todo!()
    }

    pub fn translate(&self, vpn: u64) -> Option<u64> {
        todo!()
    }
}
