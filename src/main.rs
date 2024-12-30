use std::{mem, ptr, time::Instant};

use allocator::BumpAllocator;

pub mod allocator;

const NUM_ALLOCATIONS: usize = 1_000_0000;
const BUFFER_SIZE: usize = NUM_ALLOCATIONS * mem::size_of::<u64>() * 2;

fn bump_allocator() {
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

fn standard_allocator() {
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

fn main() {
    println!("Testing Bump Allocator...");
    bump_allocator();

    println!("\nTesting Standard Allocator...");
    standard_allocator();
}
