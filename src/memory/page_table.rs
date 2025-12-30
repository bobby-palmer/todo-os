use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
enum Flag {
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

    /// Return the flags appropriate bit in a PTE repr
    fn bit(self) -> u8 {
        match self {
            Flag::Valid =>    1 << 0,
            Flag::Read =>     1 << 1,
            Flag::Write =>    1 << 2,
            Flag::Execute =>  1 << 3,
            Flag::User =>     1 << 4,
            Flag::Global =>   1 << 5,
            Flag::Accessed => 1 << 6,
            Flag::Dirty =>    1 << 7,
        }
    }
}

#[derive(Clone, Copy)]
struct Flags(u8);

impl Flags {

    /// Construct new with no flags set
    fn empty() -> Self {
        Self(0)
    }

    /// true -> flag has been added to this set
    fn contains(self, flag: Flag) -> bool {
        self.0 & flag.bit() != 0
    }

    /// Add flag to set and return the resulting set
    fn with(self, flag: Flag) -> Self {
        Self(self.0 | flag.bit())
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct Pte(u64);

impl Pte {

    fn new(ppn: u64, flags: Flags) -> Self {
        Self(ppn << 10 | flags.0 as u64)
    }

    fn flags(self) -> Flags {
        Flags((self.0 & 0xFF) as u8)
    }

    fn ppn(self) -> u64 {
        self.0 >> 10
    }
}

#[repr(transparent)]
struct PageTable([Pte; 512]);

impl PageTable {

}

impl Index<usize> for PageTable {
    type Output = Pte;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
