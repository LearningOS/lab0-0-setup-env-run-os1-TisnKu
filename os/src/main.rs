#![no_std]
#![no_main]

use crate::sbi::shutdown;

mod console;
mod lang_items;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello {}", "tianxi");
    shutdown();
}
