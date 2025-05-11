use core::fmt::Debug;

use alloc::vec::Vec;

use crate::{mm::frame_alloc, println};

use super::{
    address::{PhysAddr, PhysPageNum},
    frame_dealloc,
    page_table::PAGE_SIZE,
};

pub struct FrameAllocator {
    current: PhysPageNum,
    end: PhysPageNum,
    recycled: Vec<PhysPageNum>,
}

impl FrameAllocator {
    pub fn new(left: PhysPageNum, right: PhysPageNum) -> Self {
        assert!(left <= right, "FrameAllocator: invalid range");
        FrameAllocator {
            current: left,
            end: right,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> PhysPageNum {
        if let Some(ppn) = self.recycled.pop() {
            ppn
        } else if self.current < self.end {
            let current = self.current;
            self.current = PhysPageNum::from(current.0 + 1);
            current
        } else {
            panic!("FrameAllocator: alloc failed");
        }
    }

    pub fn dealloc(&mut self, ppn: PhysPageNum) {
        if ppn >= self.current {
            panic!("FrameAllocator: deallocate invalid ppn");
        }
        if self.recycled.contains(&ppn) {
            panic!("FrameAllocator: deallocate repeated ppn");
        }

        self.recycled.push(ppn);
    }
}

pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        let pa = PhysAddr::from(ppn);
        unsafe {
            (pa.0 as *mut u8).write_bytes(0, PAGE_SIZE);
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("FrameTracker: PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
