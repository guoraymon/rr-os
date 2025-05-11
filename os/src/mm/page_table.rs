use core::panic;

use alloc::vec::Vec;

use crate::println;

use super::{
    address::{PhysPageNum, VirtPageNum, VA_WIDTH_SV39},
    frame_alloc,
    frame_allocator::FrameTracker,
};

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 12;

pub struct PageTable {
    pub root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, target_ppn: PhysPageNum, flags: PageTableEntryFlags) {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (_, &idx) in indexes.iter().enumerate() {
            let pte_array = ppn.get_pte_array();
            let pte = &mut pte_array[idx];

            if pte.is_leaf() {
                if !pte.is_valid() {
                    *pte = PageTableEntry::new(target_ppn, flags | PageTableEntryFlags::V);
                    return;
                }
            } else if !pte.is_valid() {
                let frame = frame_alloc();
                *pte = PageTableEntry::new(frame.ppn, PageTableEntryFlags::V);
                self.frames.push(frame);
            }

            ppn = pte.ppn();
        }
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (_, &idx) in indexes.iter().enumerate() {
            let pte_array = ppn.get_pte_array();
            let pte = &mut pte_array[idx];

            if !pte.is_valid() {
                panic!("unmap failed: vpn {:?} not mapped", vpn.0);
            }

            if pte.is_leaf() {
                *pte = PageTableEntry::empty();
                return;
            }

            ppn = pte.ppn();
        }
        panic!("unmap failed: invalid page table traversal");
    }

    pub fn translate(&self, vpn: VirtPageNum) -> PageTableEntry {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (level, &idx) in indexes.iter().enumerate() {
            let pte_array = ppn.get_pte_array();
            let pte = &pte_array[idx];

            if !pte.is_valid() {
                panic!("Invalid PTE during translation at level {}", level);
            }

            if pte.is_leaf() {
                return *pte;
            }

            ppn = pte.ppn();
        }

        panic!("Translation failed: incomplete page table traversal");
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn new(ppn: PhysPageNum, flags: PageTableEntryFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.0 as usize,
        }
    }

    pub fn ppn(&self) -> PhysPageNum {
        let raw_ppn = (self.bits >> 10) & ((1 << VA_WIDTH_SV39) - 1);
        PhysPageNum::from(raw_ppn)
    }

    pub fn is_leaf(&self) -> bool {
        self.is_readable() || self.is_writeable() || self.is_executable()
    }

    pub fn is_valid(&self) -> bool {
        self.bits & 0b1 != 0
    }

    pub fn is_readable(&self) -> bool {
        self.bits & 0b01 != 0
    }

    pub fn is_writeable(&self) -> bool {
        self.bits & 0b10 != 0
    }

    pub fn is_executable(&self) -> bool {
        self.bits & 0b100 != 0
    }

    pub fn is_user(&self) -> bool {
        self.bits & 0b1000 != 0
    }
}

#[derive(Clone, Copy)]
pub struct PageTableEntryFlags(u8);

impl PageTableEntryFlags {
    pub const V: Self = Self(1 << 0);
    pub const R: Self = Self(1 << 1);
    pub const W: Self = Self(1 << 2);
    pub const X: Self = Self(1 << 3);
    pub const U: Self = Self(1 << 4);
    pub const G: Self = Self(1 << 5);
    pub const A: Self = Self(1 << 6);
    pub const D: Self = Self(1 << 7);
}

impl core::ops::BitOr for PageTableEntryFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for PageTableEntryFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
