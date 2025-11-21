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

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {

    pub fn map_at_level(
        &mut self, 
        lvl: u64,
        vpn: u64, 
        ppn: u64, 
        flags: FlagSet) -> Result<(), &'static str> 
    {
        self.map_at_level_rec(2, lvl, vpn, ppn, flags)
    }

    fn map_at_level_rec(
        &mut self, 
        current_lvl: u64,
        lvl: u64,
        vpn: u64, 
        ppn: u64, 
        flags: FlagSet) -> Result<(), &'static str> 
    {
        let idx = Self::get_vpn(vpn, current_lvl);

        if current_lvl == lvl {
            if self.0[idx].flags().contains(Flag::Valid) {
                Err("Page is already mapped at this level")
            } else {
                self.0[idx] = Pte::new(ppn, flags);
                Ok(())
            }
        } else {
            todo!()
        }
    }

    /// Unmap at the first leaf node
    pub fn unmap(&mut self, vpn: u64) -> Result<(), &'static str> {
        self.unmap_rec(2, vpn)
    }

    fn unmap_rec(&mut self, current_lvl: u64, vpn: u64) 
        -> Result<(), &'static str>
    {
        todo!()
    }

    /// Return physical page number vpn is mapped to
    pub fn translate(&self, vpn: u64) -> Option<u64> {
        todo!()
    }

    /// Get vpn index for given level
    fn get_vpn(vpn: u64, level: u64) -> usize {
        ((vpn >> (level * 9)) & 0x1FF) as usize
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        todo!()
    }
}
