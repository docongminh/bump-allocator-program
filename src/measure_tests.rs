use std::{mem, ptr, sync::Arc, thread, time::Instant};

use crate::{allocator::BumpAllocator, multi_thread_allocator::ThreadSafeBumpAllocator};

/// Define a large data to test allocations of larger sizes.
#[derive(Debug)]
struct LargeData {
    data: [u64; 128], // 1024 bytes
}

/// Test allocating varying sizes: u8, u64, and LargeStruct.
fn test_varying_allocation_sizes() {
    let num_allocations = 100_000;

    // Define different types to allocate
    let types = vec!["u8", "u64", "LargeData"];

    for typ in types {
        match typ {
            "u8" => {
                println!("Testing allocation of u8...");
                // Bump Allocator
                let size = mem::size_of::<u8>();
                let align = mem::align_of::<u8>();
                let buffer_size = num_allocations * size * 2; // Extra space for alignment
                let allocator = BumpAllocator::new(buffer_size);
                let mut allocations: Vec<*mut u8> = Vec::with_capacity(num_allocations);

                let start = Instant::now();

                for i in 0..num_allocations {
                    let ptr = allocator.allocate(size, align) as *mut u8;
                    unsafe {
                        ptr::write(ptr, i as u8);
                    }
                    allocations.push(ptr);
                }

                let duration = start.elapsed();
                println!(
                    "Bump Allocator: Allocated {} u8s in {:?}",
                    num_allocations, duration
                );

                // Standard Allocator
                let mut std_allocations: Vec<*mut u8> = Vec::with_capacity(num_allocations);
                let start = Instant::now();

                for i in 0..num_allocations {
                    let boxed = Box::new(i as u8);
                    std_allocations.push(Box::into_raw(boxed));
                }

                let duration = start.elapsed();
                println!(
                    "Standard Allocator: Allocated {} u8s in {:?}",
                    num_allocations, duration
                );

                // Cleanup
                for ptr in std_allocations {
                    unsafe {
                        Box::from_raw(ptr);
                    }
                }

                println!("----------------------------------------");
            }
            "u64" => {
                println!("Testing allocation of u64...");
                // Bump Allocator
                let size = mem::size_of::<u64>();
                let align = mem::align_of::<u64>();
                let buffer_size = num_allocations * size * 2;
                let allocator = BumpAllocator::new(buffer_size);
                let mut allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations);

                let start = Instant::now();

                for i in 0..num_allocations {
                    let ptr = allocator.allocate(size, align) as *mut u64;
                    unsafe {
                        ptr::write(ptr, i as u64);
                    }
                    allocations.push(ptr);
                }

                let duration = start.elapsed();
                println!(
                    "Bump Allocator: Allocated {} u64s in {:?}",
                    num_allocations, duration
                );

                // Standard Allocator
                let mut std_allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations);
                let start = Instant::now();

                for i in 0..num_allocations {
                    let boxed = Box::new(i as u64);
                    std_allocations.push(Box::into_raw(boxed));
                }

                let duration = start.elapsed();
                println!(
                    "Standard Allocator: Allocated {} u64s in {:?}",
                    num_allocations, duration
                );

                // Cleanup
                for ptr in std_allocations {
                    unsafe {
                        Box::from_raw(ptr);
                    }
                }

                println!("----------------------------------------");
            }
            "LargeStruct" => {
                println!("Testing allocation of LargeStruct...");
                // Bump Allocator
                let size = mem::size_of::<LargeData>();
                let align = mem::align_of::<LargeData>();
                let buffer_size = num_allocations * size * 2;
                let allocator = BumpAllocator::new(buffer_size);
                let mut allocations: Vec<*mut LargeData> = Vec::with_capacity(num_allocations);

                let start = Instant::now();

                for _ in 0..num_allocations {
                    let ptr = allocator.allocate(size, align) as *mut LargeData;
                    unsafe {
                        ptr::write(ptr, LargeData { data: [0; 128] });
                    }
                    allocations.push(ptr);
                }

                let duration = start.elapsed();
                println!(
                    "Bump Allocator: Allocated {} LargeStructs in {:?}",
                    num_allocations, duration
                );

                // Standard Allocator
                let mut std_allocations: Vec<*mut LargeData> = Vec::with_capacity(num_allocations);
                let start = Instant::now();

                for _ in 0..num_allocations {
                    let boxed = Box::new(LargeData { data: [0; 128] });
                    std_allocations.push(Box::into_raw(boxed));
                }

                let duration = start.elapsed();
                println!(
                    "Standard Allocator: Allocated {} LargeStructs in {:?}",
                    num_allocations, duration
                );

                // Cleanup
                for ptr in std_allocations {
                    unsafe {
                        Box::from_raw(ptr);
                    }
                }

                println!("----------------------------------------");
            }
            _ => {}
        }
    }
}

/// Test allocating and resetting the bump allocator multiple times.
fn test_allocation_and_reset_patterns() {
    println!("Testing allocation and reset patterns...");
    let num_allocations = 500_000;
    let iterations = 10;

    // Bump Allocator
    let size = mem::size_of::<u64>();
    let align = mem::align_of::<u64>();
    let buffer_size = num_allocations * size * 2;
    let allocator = BumpAllocator::new(buffer_size);
    let mut allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations);

    let total_start = Instant::now();

    for iter in 0..iterations {
        let start = Instant::now();

        for i in 0..num_allocations {
            let ptr = allocator.allocate(size, align) as *mut u64;
            unsafe {
                ptr::write(ptr, i as u64);
            }
            allocations.push(ptr);
        }

        let duration = start.elapsed();
        println!(
            "Bump Allocator: Iteration {} - Allocated {} u64s in {:?}",
            iter + 1,
            num_allocations,
            duration
        );

        allocator.reset();
    }

    let total_duration = total_start.elapsed();
    println!(
        "Bump Allocator: Total time for {} iterations: {:?}",
        iterations, total_duration
    );

    // Standard Allocator
    let mut std_allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations * iterations);

    let total_start = Instant::now();

    for iter in 0..iterations {
        let start = Instant::now();

        for i in 0..num_allocations {
            let boxed = Box::new(i as u64);
            std_allocations.push(Box::into_raw(boxed));
        }

        let duration = start.elapsed();
        println!(
            "Standard Allocator: Iteration {} - Allocated {} u64s in {:?}",
            iter + 1,
            num_allocations,
            duration
        );
    }

    let total_duration = total_start.elapsed();
    println!(
        "Standard Allocator: Total time for {} iterations: {:?}",
        iterations, total_duration
    );

    // Cleanup
    for ptr in std_allocations {
        unsafe {
            Box::from_raw(ptr);
        }
    }

    println!("----------------------------------------");
}

/// Test multithreaded allocations using ThreadSafeBumpAllocator and standard allocator.
fn test_multithreaded_allocations() {
    println!("Testing multithreaded allocations...");
    let num_threads = 8;
    let num_allocations_per_thread = 100_000;

    // Thread-safe Bump Allocator
    let size = mem::size_of::<u64>();
    let align = mem::align_of::<u64>();
    let buffer_size = num_threads * num_allocations_per_thread * size * 2;
    let allocator = Arc::new(ThreadSafeBumpAllocator::new(buffer_size));

    let start = Instant::now();

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let alloc_ref = Arc::clone(&allocator);
        let handle = thread::spawn(move || {
            let mut local_allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations_per_thread);
            for i in 0..num_allocations_per_thread {
                let ptr = alloc_ref.allocate(size, align) as *mut u64;
                unsafe {
                    ptr::write(ptr, i as u64);
                }
                local_allocations.push(ptr);
            }
            local_allocations
        });
        handles.push(handle);
    }

    let mut all_allocations = Vec::with_capacity(num_threads * num_allocations_per_thread);

    for handle in handles {
        let mut thread_allocations = handle.join().unwrap();
        all_allocations.append(&mut thread_allocations);
    }

    let duration = start.elapsed();
    println!(
        "ThreadSafe Bump Allocator: Allocated {} u64s across {} threads in {:?}",
        num_threads * num_allocations,
        num_threads,
        duration
    );

    allocator.reset();

    // Standard Allocator
    let start = Instant::now();

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut local_allocations: Vec<*mut u64> = Vec::with_capacity(num_allocations);
            for i in 0..num_allocations {
                let boxed = Box::new(i as u64);
                local_allocations.push(Box::into_raw(boxed));
            }
            local_allocations
        });
        handles.push(handle);
    }

    let mut all_std_allocations = Vec::with_capacity(num_threads * num_allocations);

    for handle in handles {
        let mut thread_allocations = handle.join().unwrap();
        all_std_allocations.append(&mut thread_allocations);
    }

    let duration = start.elapsed();
    println!(
        "Standard Allocator: Allocated {} u64s across {} threads in {:?}",
        num_threads * num_allocations,
        num_threads,
        duration
    );

    // Cleanup
    for ptr in all_std_allocations {
        unsafe {
            Box::from_raw(ptr);
        }
    }

    println!("----------------------------------------");
}

/// Test the bump allocator and standard allocator under stress by allocating until memory is exhausted.
fn test_stress_allocations() {
    println!("Testing stress allocations...");
    // Adjust these numbers based on your system's memory
    let allocation_size = mem::size_of::<u64>();
    let align = mem::align_of::<u64>();
    let buffer_size = 10 * 1024 * 1024; // 10 MB

    // Bump Allocator
    let allocator = BumpAllocator::new(buffer_size);
    let mut allocations: Vec<*mut u64> = Vec::new();
    let start = Instant::now();

    println!("Starting stress test for Bump Allocator...");

    loop {
        if allocations.len() % 100_000 == 0 && allocations.len() != 0 {
            println!(
                "Bump Allocator: Allocated {} u64s so far...",
                allocations.len()
            );
        }

        let ptr = match std::panic::catch_unwind(|| allocator.allocate(allocation_size, align)) {
            Ok(p) => p as *mut u64,
            Err(_) => {
                println!(
                    "Bump Allocator: Failed after allocating {} u64s.",
                    allocations.len()
                );
                break;
            }
        };

        unsafe {
            ptr::write(ptr, allocations.len() as u64);
        }
        allocations.push(ptr);
    }

    let duration = start.elapsed();
    println!(
        "Bump Allocator: Allocated {} u64s in {:?}",
        allocations.len(),
        duration
    );

    // No reset performed here as we are stressing the allocator

    // Standard Allocator
    let mut std_allocations: Vec<*mut u64> = Vec::new();
    let start = Instant::now();

    println!("Starting stress test for Standard Allocator...");

    loop {
        if std_allocations.len() % 100_000 == 0 && std_allocations.len() != 0 {
            println!(
                "Standard Allocator: Allocated {} u64s so far...",
                std_allocations.len()
            );
        }

        // Attempt to allocate; handle out-of-memory gracefully
        let boxed = Box::try_new(0u64);
        match boxed {
            Ok(b) => std_allocations.push(Box::into_raw(b)),
            Err(_) => {
                println!(
                    "Standard Allocator: Failed after allocating {} u64s.",
                    std_allocations.len()
                );
                break;
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "Standard Allocator: Allocated {} u64s in {:?}",
        std_allocations.len(),
        duration
    );

    // Cleanup
    for ptr in std_allocations {
        unsafe {
            Box::from_raw(ptr);
        }
    }

    println!("----------------------------------------");
}
