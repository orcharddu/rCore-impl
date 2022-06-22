#![no_std]
#![no_main]
#![feature(core_panic)]
#![feature(panic_info_message)]

mod handler;
mod sbi;
mod console;

use core::{arch::global_asm};
global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    let sbss = sbss as usize;
    let ebss = ebss as usize;
    (sbss..ebss).for_each(|x| unsafe {
        (x as *mut u8).write_volatile(0);
    })
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello world");
    panic!("Shutdown machine");
}

