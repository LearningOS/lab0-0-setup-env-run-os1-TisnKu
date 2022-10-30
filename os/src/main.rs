#![no_std]
#![no_main]
#![feature(panic_info_message)]

use log::{debug, error, info, trace, warn};

use crate::sbi::shutdown;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("hello {}", "tianxi");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    warn!(
        ".boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    panic!("shut down machine");
}
