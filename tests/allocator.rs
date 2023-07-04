#![feature(strict_provenance)]
use stackalloc::StackAllocator;

#[test]
fn simple() {
    let allocator: StackAllocator<8, 2> = StackAllocator::new();

    let first = allocator.malloc(4);
    let second = allocator.malloc(4);

    let first_addr = allocator.mempool_addr();

    assert_eq!(first.addr(), first_addr);
    assert_eq!(second.addr(), first_addr + 4);

    allocator.free(first);
    let third = allocator.malloc(4);

    assert_eq!(third.addr(), first_addr);
}

#[test]
fn complex() {
    let allocator: StackAllocator<8, 4> = StackAllocator::new();
    let first_addr = allocator.mempool_addr();

    let first = allocator.malloc(4);
    allocator.malloc(4);

    allocator.free(first);

    let third = allocator.malloc(2);
    let fourth = allocator.malloc(2);

    assert_eq!(third.addr(), first_addr);
    assert_eq!(fourth.addr(), first_addr + 2);
}
