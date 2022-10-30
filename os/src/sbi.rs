fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
                "ecall",
                inlateout("x10") arg0 => ret,
                in("x11") arg1,
                in("x12") arg2,
                in("x17") which,
        )
    }
    ret
}

const SBI_SHUTDOWN: usize = 8;
const SBI_PUT_CHAR: usize = 1;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown");
}
pub fn console_putchar(c: char) {
    sbi_call(SBI_PUT_CHAR, c as usize, 0, 0);
}
