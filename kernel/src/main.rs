#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

#[bootshim::entry]
fn kmain() -> ! {
    loop {
    }
}

#[panic_handler]
fn on_panic(_: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe"); }
    }
}
