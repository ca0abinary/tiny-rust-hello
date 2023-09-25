use core::arch::asm;

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    asm!("mov fp, sp", "mov x0, fp", "bl get_args", options(noreturn))
}

pub unsafe fn sys_exit(exit_code: usize) -> ! {
    asm!("svc 0",
        in("w8") 93,
        in("x0") exit_code,
        options(nostack, noreturn)
    )
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    asm!("svc #0",
        inout("x0") 1 => _,
        inout("x1") buffer => _,
        inout("x2") count => _,
        inout("x8") 64 => _,
        options(nostack)
    )
}
