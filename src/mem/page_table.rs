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

/// A collection of permission flags as a bitset
#[derive(Clone, Copy)]
pub struct FlagSet(u64);

impl FlagSet {
    pub fn empty() -> Self {
        Self(0)
    }

    fn contains(&self, flag: Flag) -> bool {
        self.0 & flag.bit() != 0
    }

    /// Ppn points to a mapped page, not a page table
    fn is_leaf(&self) -> bool {
        self.contains(Flag::Valid) && (
            self.contains(Flag::Read)
            | self.contains(Flag::Write) 
            | self.contains(Flag::Execute)
        )
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

/// Page table level (determines how much is covered)
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum MapLevel {
    /// 1GB
    Huge,
    /// 2MB
    Big,
    /// one page (4KB)
    Page,
}

impl MapLevel {
    fn get_index(&self, vpn: u64) -> usize {
        let index = self.index();
        ((vpn >> (9 * index)) & 0x1FF) as usize
    }

    fn index(&self) -> usize {
        match self {
            MapLevel::Huge => 2,
            MapLevel::Big => 1,
            MapLevel::Page => 0,
        }
    }
}

impl Iterator for MapLevel {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MapLevel::Huge => Some(Self::Big),
            MapLevel::Big => Some(Self::Page),
            MapLevel::Page => None,
        }
    }
}

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {

    pub fn map(&mut self, lvl: MapLevel, vpn: u64, ppn: u64, flags: FlagSet) -> Result<(), &'static str> {
        todo!()
    }

    pub fn unmap(&mut self, vpn: u64) -> Result<(), &'static str> {
        todo!()
    }

    pub fn translate(&self, vpn: u64) -> Option<u64> {
        todo!()
    }

    pub fn translate_rec(&self, vpn: u64, current_level: MapLevel) -> Option<u64> {
        let index = current_level.get_index(vpn);
        let pte = self.0[index];
        let flags = pte.flags();

        if flags.contains(Flag::Valid) {
            if flags.is_leaf() {
                todo!()
            } else {
                todo!()
            }
        } else {
            None
        }
    }
}
