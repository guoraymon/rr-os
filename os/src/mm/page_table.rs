use super::{
    address::{PhysPageNum, VirtPageNum},
    frame_alloc,
};

pub struct PageTableEntryFlags(u8);

pub struct PageTable {
    pub root_ppn: PhysPageNum,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc();
        PageTable {
            root_ppn: frame.ppn,
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PageTableEntryFlags) {}

    pub fn unmap(&mut self, vpn: VirtPageNum) {}
}
