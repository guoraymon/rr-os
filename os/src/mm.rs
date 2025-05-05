use core::{alloc::GlobalAlloc, cell::UnsafeCell, panic};

use crate::println;

pub struct EarlyAllocator {
    pub inner: UnsafeCell<EarlyAllocatorInner>,
}

pub struct EarlyAllocatorInner {
    pub start: usize,
    pub end: usize,
    pub pos: usize,
}

impl EarlyAllocator {
    fn init(&self, start: usize, end: usize) {
        unsafe {
            *(self.inner.get()) = EarlyAllocatorInner {
                start,
                end,
                pos: start,
            }
        }
    }
}

unsafe impl GlobalAlloc for EarlyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let inner = &mut *(self.inner.get());
        if inner.pos + layout.size() > inner.end {
            panic!("out of memory");
        }
        inner.pos += layout.size();
        inner.pos as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, layout: core::alloc::Layout) {
        let inner = &mut *(self.inner.get());
        if inner.pos - layout.size() < inner.start {
            panic!("invalid dealloc");
        }
        inner.pos -= layout.size();
    }
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: EarlyAllocator = EarlyAllocator {
    inner: UnsafeCell::new(EarlyAllocatorInner {
        start: 0,
        end: 0,
        pos: 0,
    }),
};

static mut HEAP_SPACE: [u8; 0x200_0000] = [0; 0x200_0000];

pub fn init() {
    unsafe {
        GLOBAL_ALLOCATOR.init(
            HEAP_SPACE.as_ptr() as usize,
            HEAP_SPACE.as_ptr() as usize + HEAP_SPACE.len(),
        );
    }
}

pub fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn __bss_start();
        fn __bss_end();
    }
    let bss_range = __bss_start as usize..__bss_end as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for i in 0..500 {
        assert_eq!(v[i], i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    println!("heap_test passed!");
}