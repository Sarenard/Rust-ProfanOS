use core::alloc::{GlobalAlloc, Layout};

use crate::libs::libc::{free, malloc};

pub struct MemoryAllocator;

unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size() as usize) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) };
    }
}

#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;
