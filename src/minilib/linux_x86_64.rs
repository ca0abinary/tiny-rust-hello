use core::arch::asm;

pub unsafe fn exit(exit_code:usize) -> ! {
    asm!("syscall",
            in("rax") 60,
            in("rdi") exit_code,
            options(nostack, noreturn)
    )
}

pub unsafe fn print(string:&'static str) {
    write(string.as_ptr(), string.len())
}

pub unsafe fn write(buffer: *const u8, count: usize) {
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
