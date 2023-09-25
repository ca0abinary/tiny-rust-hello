use core::arch::asm;

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    // Move the stack pointer before it gets clobbered
    asm!("mov rdi, rsp", "call get_args", options(noreturn))
}

pub unsafe fn sys_exit(exit_code: usize) -> ! {
    asm!("syscall",
            in("rax") 60,
            in("rdi") exit_code,
            options(nostack, noreturn)
    )
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    asm!("syscall",
            inout("rax") 1 => _,
            in("rdi") 1,
            in("rsi") buffer,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
    )
}
