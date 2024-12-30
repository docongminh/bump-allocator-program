use anchor_lang::solana_program::entrypoint::HEAP_START_ADDRESS;
use std::{alloc::Layout, mem::size_of, ptr::null_mut};

/// Length of the memory region used for program heap.
pub const HEAP_LENGTH: usize = 8 * 32 * 1024; // 256 KB

pub struct CustomizeBumpAllocator;

unsafe impl std::alloc::GlobalAlloc for CustomizeBumpAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        const POS_PTR: *mut usize = HEAP_START_ADDRESS as *mut usize;
        const TOP_ADDRESS: usize = HEAP_START_ADDRESS as usize + HEAP_LENGTH;
        const BOTTOM_ADDRESS: usize = HEAP_START_ADDRESS as usize + size_of::<*mut u8>();

        let mut pos = *POS_PTR;
        if pos == 0 {
            // First time, set starting position to bottom address
            pos = BOTTOM_ADDRESS;
        }

        // Align the position upwards
        pos = (pos + layout.align() - 1) & !(layout.align() - 1);
        let next_pos = pos.saturating_add(layout.size());

        if next_pos > TOP_ADDRESS {
            return null_mut();
        }

        *POS_PTR = next_pos;
        pos as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // Bump allocator does not support deallocation
    }
}
