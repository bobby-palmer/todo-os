//! RISCV-64 Sv48 Page tables

use core::{ops::{Add, Sub}, ptr::NonNull};

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
    fn to_bit(self) -> u64 {
        let shift = match self {
            Flag::Valid => 0,
            Flag::Read => 1,
            Flag::Write => 2,
            Flag::Execute => 3,
            Flag::User => 4,
            Flag::Global => 5,
            Flag::Accessed => 6,
            Flag::Dirty => 7,
        };
        1 << shift
    }
}

pub struct Flags(u64);

impl Flags {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn with(self, flag: Flag) -> Self {
        Self(self.0 | flag.to_bit())
    }

    pub fn remove(self, flag: Flag) -> Self {
        Self(self.0 & !flag.to_bit())
    }

    pub fn contains(&self, flag: Flag) -> bool {
        self.0 & flag.to_bit() != 0
    }

    fn is_leaf(&self) -> bool {
        self.contains(Flag::Valid) && (
            self.contains(Flag::Read)
            | self.contains(Flag::Write) 
            | self.contains(Flag::Execute)
        )
    }
}

impl Add<Flag> for Flags {
    type Output = Self;

    fn add(self, rhs: Flag) -> Self::Output {
        self.with(rhs)
    }
}

impl Sub<Flag> for Flags {
    type Output = Self;

    fn sub(self, rhs: Flag) -> Self::Output {
        self.remove(rhs)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pte(u64);

impl Pte {

    pub fn new(ppn: u64, flags: Flags) -> Self {
        Self((ppn << 10) | flags.0)
    }

    pub fn ppn(&self) -> u64 {
        self.0 >> 10
    }

    pub fn flags(&self) -> Flags {
        Flags(self.0 & 0xFF)
    }

    fn is_leaf(&self) -> bool {
        self.flags().is_leaf()
    }
}

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {

    pub fn get_current() -> NonNull<PageTable> {
        todo!()
    }

    /// Make a 1GB mapping (typically used for kernel only)
    pub fn map_huge(&mut self, vpn: u64, pte: Pte) {
        self.0[Self::index_vpn(vpn, 2)] = pte;
    }

    fn index_vpn(vpn: u64, level: u8) -> usize {
        ((vpn >> (9 * level)) & 0x1FF) as usize
    }
}
