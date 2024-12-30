use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::time::Instant;
use std::{mem, ptr, usize};

use crate::{align_up, BUFFER_SIZE, NUM_ALLOCATIONS};

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

pub fn measure_bump_allocator() {
    let allocator = BumpAllocator::new(BUFFER_SIZE);
    let mut allocations: Vec<*mut u64> = Vec::with_capacity(NUM_ALLOCATIONS);
    let start = Instant::now();
    for i in 0..NUM_ALLOCATIONS {
        let pointer = allocator.allocate(mem::size_of::<u64>(), mem::size_of::<u64>()) as *mut u64;
        unsafe {
            ptr::write(pointer as *mut u64, i as u64);
        }
        allocations.push(pointer);
    }
    let duration = start.elapsed();

    println!(
        "Bump Allocator: Allocated {} u64 in {:?}",
        NUM_ALLOCATIONS, duration
    );
}

pub fn measure_standard_allocator() {
    let mut allocations: Vec<*mut u64> = Vec::with_capacity(NUM_ALLOCATIONS);

    let start = Instant::now();
    for i in 0..NUM_ALLOCATIONS {
        let boxed = Box::new(i as u64);
        allocations.push(Box::into_raw(boxed));
    }

    let duration = start.elapsed();
    println!(
        "Standard Allocator: Allocated {} u64 in {:?}",
        NUM_ALLOCATIONS, duration
    );

    // clean up
    for ptr in allocations {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}
