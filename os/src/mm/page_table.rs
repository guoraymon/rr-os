use core::panic;

use alloc::vec::Vec;

use super::{
    address::{PhysPageNum, VirtPageNum},
    frame_alloc,
    frame_allocator::FrameTracker,
};

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

        for (level, &idx) in indexes.iter().enumerate() {
            let pte_array = ppn.get_page_table_entries();
            let pte = &mut pte_array[idx];

            if level == 2 {
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

        for (level, &idx) in indexes.iter().enumerate() {
            let pte_array = ppn.get_page_table_entries();
            let pte = &mut pte_array[idx];

            if !pte.is_valid() {
                panic!("unmap failed: vpn {:?} not mapped", vpn.0);
            }

            if level == 2 {
                *pte = PageTableEntry { bits: 0 };
                return;
            }

            ppn = pte.ppn();
        }
        panic!("unmap failed: invalid page table traversal");
    }
}

pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PageTableEntryFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.0 as usize,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.bits & 1 != 0
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum::from(self.bits >> 10)
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
