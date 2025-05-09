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
}

#[derive(Clone, Copy)]
pub struct VirtPageNum {
    pub value: usize,
}
