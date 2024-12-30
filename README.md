# Result experiment bump heap allocation

## Varying allocation sizes
```rust
U8
Bump Allocator: allocated 100000 u8s in 12.706167ms
Standard Allocator: allocated 100000 u8s in 8.157791ms


U64
Bump Allocator: allocated 100000 u64s in 4.477583ms
Standard Allocator: allocated 100000 u64s in 5.18125ms


LargeData
Bump Allocator: allocated 100000 LargeData in 15.069917ms
Standard Allocator: allocated 100000 LargeData in 13.71225ms
```

## Allocation and reset patterns
```rust
BUMP ALLOCATOR
Iteration 1 - Allocated 500000 u64s in 13.384208ms
Iteration 2 - Allocated 500000 u64s in 12.581833ms
Iteration 3 - Allocated 500000 u64s in 13.291667ms
Iteration 4 - Allocated 500000 u64s in 12.516875ms
Iteration 5 - Allocated 500000 u64s in 12.418875ms
Iteration 6 - Allocated 500000 u64s in 12.381667ms
Iteration 7 - Allocated 500000 u64s in 13.034125ms
Iteration 8 - Allocated 500000 u64s in 13.035541ms
Iteration 9 - Allocated 500000 u64s in 12.711042ms
Iteration 10 - Allocated 500000 u64s in 12.285292ms

Total time for 10 iterations: 127.712792ms

STANDARD ALLOCATOR
Iteration 1 - Allocated 500000 u64s in 15.481875ms
Iteration 2 - Allocated 500000 u64s in 16.976959ms
Iteration 3 - Allocated 500000 u64s in 15.765417ms
Iteration 4 - Allocated 500000 u64s in 15.657334ms
Iteration 5 - Allocated 500000 u64s in 15.69975ms
Iteration 6 - Allocated 500000 u64s in 15.601125ms
Iteration 7 - Allocated 500000 u64s in 15.457875ms
Iteration 8 - Allocated 500000 u64s in 15.491416ms
Iteration 9 - Allocated 500000 u64s in 15.664917ms
Iteration 10 - Allocated 500000 u64s in 15.508875ms

Total time for 10 iterations: 157.383958ms
```

# TODO
- result test multi thread
