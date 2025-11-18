//! RISCV-64 Sv48 Page tables

use core::{ops::{Add, Sub}, ptr::NonNull};

use crate::mem::{common::{Page, PAGE_SIZE, PHYSICAL_RAM_START, VIRTUAL_RAM_START}, FREE_PAGES};

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
struct Pte(u64);

impl Pte {

    fn new(ppn: u64, flags: Flags) -> Self {
        Self((ppn << 10) | flags.0)
    }

    fn ppn(&self) -> Option<u64> {
        if self.is_valid() {
            Some(self.0 >> 10)
        } else {
            None
        }
    }

    fn flags(&self) -> Flags {
        Flags(self.0 & 0xFF)
    }

    fn is_valid(&self) -> bool {
        self.0 & 1 == 1
    }
}

#[repr(C)]
pub struct PageTable([Pte; 512]);

impl PageTable {
    pub const MAX_LEVEL: u8 = 2;

    fn new_raw() -> Option<NonNull<Self>> {
        if let Some(mut page) = FREE_PAGES.lock().pop() {
            unsafe {page.as_mut().0.fill(0);}
            Some(page.cast())
        } else {
            None
        }
    }

    pub fn translate_addr(&self, vaddr: usize) -> Option<usize> {
        let vpn = (vaddr / PAGE_SIZE) as u64;
        if let Some(ppn) = self.translate_page(vpn) {
            Some((vaddr % PAGE_SIZE) | (ppn as usize * PAGE_SIZE))
        } else {
            None
        }
    }

    pub fn translate_page(&self, vpn: u64) -> Option<u64> {
        let (pte, level) = self.get_lowest_pte(vpn, Self::MAX_LEVEL);
        let mask: u64 = (1 << (9 * level)) - 1;

        if pte.flags().is_leaf() {
            Some(pte.ppn().unwrap() | (vpn & mask))
        } else {
            None
        }
    }

    pub fn map_at_level(&mut self, vpn: u64, ppn: u64, flags: Flags, level: u8) -> Result<(), &'static str> {
        while let (pte, current_level) = self.get_lowest_pte_mut(vpn, Self::MAX_LEVEL) && current_level >  level {

            if pte.flags().is_leaf() {
                return Err("Already mapped");
            }

            // TODO return error here
            let child = Self::new_raw().unwrap(); 
            let ppn = unsafe {child.cast::<Page>().as_ref().ppn()};
            *pte = Pte::new(ppn, Flags::empty() + Flag::Valid);
        }

        let (pte, current_level) = self.get_lowest_pte_mut(vpn, Self::MAX_LEVEL);

        if current_level != level {
            Err("Cannot map at this level")
        } else if pte.is_valid() {
            Err("Already mapped")
        } else {
            *pte = Pte::new(ppn, flags);
            Ok(())
        }
    }

    pub fn unmap(&mut self, vpn: u64) -> Result<(), &'static str> {
        let (pte, _) = self.get_lowest_pte_mut(vpn, Self::MAX_LEVEL);

        if pte.flags().is_leaf() {
            *pte = Pte::new(0, Flags::empty());
            Ok(())
        } else {
            Err("Vpn not mapped")
        }
    }

    pub fn set_flags(&mut self, vpn: u64, flags: Flags) -> Result<(), &'static str> {
        let (pte, _) = self.get_lowest_pte_mut(vpn, Self::MAX_LEVEL);

        if pte.flags().is_leaf() {
            let ppn = pte.ppn().unwrap();
            *pte = Pte::new(ppn, flags);
            Ok(())
        } else {
            Err("Vpn not mapped")
        }
    }

    fn get_lowest_pte_mut<'a>(&'a mut self, vpn: u64, current_level: u8) -> (&'a mut Pte, u8) {
        let pte = &mut self.0[Self::index_vpn(vpn, current_level)];
        let flags = pte.flags();
        
        if flags.contains(Flag::Valid) && !flags.is_leaf() {
            let page = Page::from_ppn(pte.ppn().unwrap());
            let mut child = page.cast::<PageTable>();
            unsafe {
                child.as_mut().get_lowest_pte_mut(vpn, current_level - 1)
            }
        } else {
            (pte, current_level)
        }
    }

    fn get_lowest_pte<'a>(&'a self, vpn: u64, current_level: u8) -> (&'a Pte, u8) {
        let pte = &self.0[Self::index_vpn(vpn, current_level)];
        let flags = pte.flags();
        
        if flags.contains(Flag::Valid) && !flags.is_leaf() {
            let page = Page::from_ppn(pte.ppn().unwrap());
            let child = page.cast::<PageTable>();
            unsafe {
                child.as_ref().get_lowest_pte(vpn, current_level - 1)
            }
        } else {
            (pte, current_level)
        }
    }

    fn index_vpn(vpn: u64, level: u8) -> usize {
        ((vpn >> (9 * level)) & 0x1FF) as usize
    }
}
