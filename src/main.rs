pub mod allocator;
pub mod measure_tests;
pub mod multi_thread_allocator;

use measure_tests::{measure_allocation_and_reset_patterns, measure_varying_allocation_sizes};

pub fn align_up(address: usize, align: usize) -> usize {
    (address + align - 1) & !(align - 1)
}

fn main() {
    // Measure varying allocation sizes
    measure_varying_allocation_sizes();

    // Measure allocation and reset patterns
    measure_allocation_and_reset_patterns();
}
