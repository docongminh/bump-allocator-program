use std::{mem, ptr, time::Instant};

use allocator::{measure_bump_allocator, measure_standard_allocator};

pub mod allocator;
pub mod multi_thread_allocator;
pub mod measure_tests;

const NUM_ALLOCATIONS: usize = 1_000_0000;
const BUFFER_SIZE: usize = NUM_ALLOCATIONS * mem::size_of::<u64>() * 2;

pub fn align_up(address: usize, align: usize) -> usize {
    (address + align - 1) & !(align - 1)
}

fn main() {
    println!("Testing Bump Allocator...");
    measure_bump_allocator();

    println!("\nTesting Standard Allocator...");
    measure_standard_allocator();


}
