use super::page_table::PageTableEntry;

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
pub const PAGE_SIZE: usize = 0x1000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        assert!(value < (1 << PA_WIDTH_SV39), "PhysAddr out of range");
        Self(value)
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysPageNum(pub usize);

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl PhysPageNum {
    pub fn addr(&self) -> PhysAddr {
        PhysAddr(self.0 * PAGE_SIZE)
    }

    pub fn get_page_table_entries(&self) -> &mut [PageTableEntry] {
        unsafe { core::slice::from_raw_parts_mut(self.addr().0 as *mut PageTableEntry, 512) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        assert!(value < (1 << VA_WIDTH_SV39), "VirtAddr out  of range!");
        Self(value)
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtPageNum(pub usize);

impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        [
            (self.0) & 0x1ff,
            (self.0 >> 9) & 0x1ff,
            (self.0 >> 18) & 0x1ff,
        ]
    }
}
