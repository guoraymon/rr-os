use super::page_table::PageTableEntry;

pub const PAGE_SIZE: usize = 0x1000;

#[derive(Clone, Copy)]
pub struct PhysAddr {
    pub value: usize,
}

impl PhysAddr {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum::new(self.value / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum::new((self.value + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Clone, Copy)]
pub struct VirtAddr {
    pub value: usize,
}

impl VirtAddr {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum::new(self.value / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum::new((self.value + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysPageNum {
    pub value: usize,
}

impl PhysPageNum {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.value * PAGE_SIZE)
    }

    pub fn get_page_table_entries(&self) -> &mut [PageTableEntry] {
        unsafe { core::slice::from_raw_parts_mut(self.addr().value as *mut PageTableEntry, 512) }
    }
}

#[derive(Clone, Copy)]
pub struct VirtPageNum {
    pub value: usize,
}

impl VirtPageNum {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn indexes(&self) -> [usize; 3] {
        [
            (self.value) & 0x1ff,
            (self.value >> 9) & 0x1ff,
            (self.value >> 18) & 0x1ff,
        ]
    }
}
