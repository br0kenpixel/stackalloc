# Stack Allocator

This is a simple stack allocator for Rust. Its slightly more complex than a bump allocator, since it allows deallcating individual allocations.
It works by keeping track of individual allocations in a second array.

**⚠️ Note that this is a highly experimental allocator and should not be used in production.**

- Disadvantages:
    - While the allocator library itself compiles in stable Rust, using it will likely require nightly Rust.
    - Since the array used to keep track of allocations must also be statically sized, the allocator can only create a fixed number of allocations.
    - With a small memory size and a large number of (de)allocations, there is a high chance of fragmentation.
    - I'm not sure about thread safety, but since it does compile, I'd assume it is.

It can however be useful on embedded systems, where a bump allocator may not be sufficient due to not being able to free individual allocations.

## Pseudo-example
```rust
#![no_std]
extern crate alloc;

#[global_allocator]
static mut ALLOCATOR: StackAllocator<128, 5> = StackAllocator::new();

...
```

# Source code
[`lib.rs`](src/lib.rs) contains the actual allocator implementation. [`alloregion.rs`](src/alloregion.rs) contains the `AllocRegion` struct, which is used to keep track of allocation metadata. [`memcell.rs`](src/memcell.rs) contains the `MemCell` struct, which is similar to a [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html#) but has a `get_mut()` method that returns a mutable reference to the value inside the `MemCell` without requiring a mutable reference to the `MemCell` itself. It also restricts the type to a threadsafe [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)able and [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)able type.

## Generic parameters
**StackAllocator<MEMSIZE, NALLOCS>**
- `MEMSIZE`: The size of the memory that the allocator can use.
- `NALLOCS`: The maximum number of allocations that can be made.