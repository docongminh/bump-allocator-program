use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::sync::Mutex;
use std::{mem, usize};
use crate::align_up;

pub struct ThreadSafeBumpAllocator {
    buffer: *mut u8,
    capacity: usize,
    offset: Mutex<usize>,
}

impl ThreadSafeBumpAllocator {
    pub fn new(capacity: usize) -> Self {
        let layout =
            Layout::from_size_align(capacity, mem::align_of::<usize>()).expect("Invalid layout");
        let buffer = unsafe { alloc(layout) };
        if buffer.is_null() {
            panic!("failed to allocate buffer for PumpAllocator");
        }

        ThreadSafeBumpAllocator {
            buffer,
            capacity,
            offset: Mutex::new(0),
        }
    }

    pub fn allocate(&self, size: usize, align: usize) -> *mut u8 {
        let mut current_offset = self.offset.lock().unwrap();
        let start = align_up(*current_offset, align);
        let end = start
            .checked_add(size)
            .expect("overflow in allocation size");

        if end > self.capacity {
            panic!("BumpAllocator out of memory")
        }

        *current_offset = end;
        unsafe { self.buffer.add(start) }
    }

    pub fn reset(&self) {
        let mut current_offset = self.offset.lock().unwrap();
        *current_offset = 0;
    }
}

impl Drop for ThreadSafeBumpAllocator {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.capacity, mem::align_of::<usize>())
            .expect("Invalid layout");
        unsafe {
            dealloc(self.buffer, layout);
        }
    }
}