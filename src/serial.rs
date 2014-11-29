use utils::{inb, outb};
use super::core::slice::{SlicePrelude};
use super::core::iter::Iterator;
use super::core::str::{StrPrelude};
use super::core::option::{Some, None};

static PORT: u16 = 0x3f8;    /* COM1 */

pub fn init() {
    unsafe {
        outb(PORT + 1, 0x00);    // Disable all interrupts
        outb(PORT + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(PORT + 1, 0x00);    //                  (hi byte)
        outb(PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set
    }
}

fn is_transmit_empty() -> bool {
    return unsafe { (inb(PORT + 5) & 0x20) != 0 };
}

fn write_byte(b: u8) {
    while (!is_transmit_empty()) {}
    unsafe {
        outb(PORT, b);
    }
}

pub fn write(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}
