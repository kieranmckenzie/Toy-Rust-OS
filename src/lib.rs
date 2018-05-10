#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods, const_fn)]
#![no_builtins]
#![no_std]

//pub mod lang_items;
//use core::ptr::{read_volatile, write_volatile};

extern crate volatile;
mod common;
mod gpio;
mod stdio;
mod timer;
mod uart;
use core::fmt::Write;
use core::sync::atomic::AtomicBool;
use gpio::*;
use stdio::{stdin, stdout};
use timer::spin_sleep_millis;
use uart::Uart;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut uart = Uart::new();
    let mut ch = 'a';
    loop {
        stdout().unwrap().write_char(ch).unwrap();
        for &c in stdout().unwrap().into_iter() {
            uart.write_byte(c);
        }
        uart.write_str("\r\n").unwrap();
        if stdout().unwrap().len() > 70 {
            stdout().unwrap().clear();
            if ch == 'z' {
                ch = 'a';
            } else {
                ch = (ch as u8 + 1) as char
            }
        }
    }
}
