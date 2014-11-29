#![allow(ctypes)]
#![no_std]
#![feature(asm)]
#![feature(macro_rules)]

extern crate core;
pub mod idt;
pub mod kbd;
pub mod mem;
pub mod pic;
pub mod serial;
pub mod stdio;
pub mod utils;

#[no_mangle]
pub unsafe fn main() {
    stdio::clear_screen();
    serial::init();
    idt::idt_install();
    serial::write("Starting...\n");
}
