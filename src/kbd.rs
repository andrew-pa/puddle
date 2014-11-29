use utils::{outb, inb};
use core::option::{Option, Some, None};
use core::slice::{SlicePrelude};
use stdio;

static mut shifted: bool = false;

pub static SCAN_CODE_MAPPING: &'static [u8] = b"\
\x00\x1B1234567890-=\x08\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?????????????789-456+1230.?????";
pub static SCAN_CODE_MAPPING_SHIFTED: &'static [u8] = b"\
\x00\x1B!@#$%^&*()_+\x08\tQWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?????????????789-456+1230.?????";


#[no_mangle]
pub unsafe fn _interrupt_handler_kbd() {
    let scancode = inb(0x60);
    change_state(scancode);
    match get_char(scancode) {
        None => {},
        Some(8) => stdio::backspace(),
        Some(10) => stdio::newline(),
        Some(c) => stdio::putc(c)
    };
    outb(0x20, 0x20);
}


pub fn change_state(scancode: u8) {
    let is_keydown: bool = scancode & 0x80 == 0;
    if is_keydown {
        match scancode {
            0x2A | 0x36 => unsafe { shifted = true },
            _ => {}
        }
    } else {
        let scancode_lower = scancode & !0x80u8;
        match scancode_lower {
            0x2A | 0x36 => unsafe { shifted = false },
            _ => {}
        }
    }
}

pub fn get_char(scancode: u8) -> Option<u8> {
    if (scancode >= SCAN_CODE_MAPPING.len() as u8
        || SCAN_CODE_MAPPING[scancode as uint] == ('?' as u8)
        || scancode < 2
        || scancode > 63) {
        return None;
    }
    unsafe {
        if shifted {
            Some(SCAN_CODE_MAPPING_SHIFTED[scancode as uint])
        } else {
            Some(SCAN_CODE_MAPPING[scancode as uint])
        }
    }
}
