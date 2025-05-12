mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
mod page_table;

use address::{PhysAddr, PhysPageNum};
use frame_allocator::{FrameAllocator, FrameTracker};
use heap_allocator::EarlyAllocator;
use memory_set::MemorySet;

use crate::utils::{LazyLock, SyncRefCell};

pub const MEMORY_END: usize = 0x8800_0000;

#[global_allocator]
static mut HEAP_ALLOCATOR: EarlyAllocator = EarlyAllocator::new();

// Must be declared as mutable to place it in the .bss section instead of .rodata
static mut HEAP_SPACE: [u8; 0x200_0000] = [0; 0x200_0000];

pub fn heap_init() {
    unsafe {
        HEAP_ALLOCATOR.init(
            HEAP_SPACE.as_ptr() as usize,
            HEAP_SPACE.as_ptr() as usize + HEAP_SPACE.len(),
        );
    }
}

extern "C" {
    fn __kernel_end();
}

static FRAME_ALLOCATOR: LazyLock<SyncRefCell<FrameAllocator>> = LazyLock::new(|| {
    SyncRefCell::new({
        FrameAllocator::new(
            PhysAddr::from(__kernel_end as usize).ceil(),
            PhysAddr::from(MEMORY_END).floor(),
        )
    })
});

pub fn frame_alloc() -> FrameTracker {
    FrameTracker::new(FRAME_ALLOCATOR.borrow_mut().alloc())
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.borrow_mut().dealloc(ppn);
}

static KERNEL_SPACE: LazyLock<MemorySet> = LazyLock::new(|| MemorySet::new_kernel());

pub fn init() {
    heap_init();
    KERNEL_SPACE.activate();
}
