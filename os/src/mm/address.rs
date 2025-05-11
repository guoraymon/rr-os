use super::page_table::{PageTableEntry, PAGE_SIZE, PAGE_SIZE_BITS};

pub const PA_WIDTH_SV39: usize = 56;
pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
pub const VA_WIDTH_SV39: usize = 39;
pub const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        assert!(value < (1 << PA_WIDTH_SV39), "PhysAddr out of range");
        Self(value)
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(value: PhysPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
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
        assert!(value < (1 << PPN_WIDTH_SV39), "PhysPageNum out  of range!");
        Self(value)
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &mut [PageTableEntry] {
        unsafe {
            core::slice::from_raw_parts_mut(PhysAddr::from(*self).0 as *mut PageTableEntry, 512)
        }
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
        assert!(value < (1 << VPN_WIDTH_SV39), "VirtPageNum out of range");
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
