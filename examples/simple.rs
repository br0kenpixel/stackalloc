#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate alloc;
use alloc::{format, string::String, vec, vec::Vec};
use panic_abort as _;
use stackalloc::StackAllocator;

#[global_allocator]
static mut ALLOCATOR: StackAllocator<128, 5> = StackAllocator::new();

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let v: Vec<i32> = vec![9, 6, -23];
    let s = String::from("Hello, World!\n");
    let _vecinfo = format!("{v:#?}\n");

    assert_eq!(format!("{v:?}"), "[9, 6, -23]");
    assert_eq!(s, "Hello, World!\n");
    0
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[link(name = "c")]
extern "C" {}
