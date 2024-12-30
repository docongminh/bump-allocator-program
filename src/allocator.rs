use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::{mem, usize};

fn align_up(address: usize, align: usize) -> usize {
    (address + align - 1) & !(align - 1)
}
pub struct BumpAllocator {
    buffer: *mut u8,
    capacity: usize,
    offset: RefCell<usize>,
}

impl BumpAllocator {
    pub fn new(capacity: usize) -> Self {
        let layout =
            Layout::from_size_align(capacity, mem::align_of::<usize>()).expect("Invalid layout");
        let buffer = unsafe { alloc(layout) };
        if buffer.is_null() {
            panic!("failed to allocate buffer for PumpAllocator");
        }

        BumpAllocator {
            buffer,
            capacity,
            offset: RefCell::new(0),
        }
    }

    pub fn allocate(&self, size: usize, align: usize) -> *mut u8 {
        let mut current_offset = self.offset.borrow_mut();
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
        let mut current_offset = self.offset.borrow_mut();
        *current_offset = 0;
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.capacity, mem::align_of::<usize>())
            .expect("Invalid layout");
        unsafe {
            dealloc(self.buffer, layout);
        }
    }
}
