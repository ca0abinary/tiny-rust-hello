use core::arch::asm;

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    asm!("mov fp, sp", "mov x0, fp", "bl get_args", options(noreturn))
}

pub unsafe fn sys_exit(exit_code: usize) -> ! {
    asm!(
        "svc 0",
        in("w8") 93,
        in("x0") exit_code,
        options(nostack, noreturn)
    )
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    asm!(
        "mov  x0, #1",
        "mov  x1, {buffer}",
        "mov  x2, {count}",
        "mov  x8, #64",
        "svc #0",
        buffer = inout(reg) buffer => _,
        count = inout(reg) count => _,
        out("x0") _,
        out("x8") _,
        options(nostack)
    )
}
