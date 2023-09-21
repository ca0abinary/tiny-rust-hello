use core::arch::asm;

pub fn exit(exit_code:i32) -> ! {
    unsafe {
        asm!("syscall",
             in("eax") 60,
             in("edi") exit_code,
             options(nostack, noreturn)
        )
    }
}

pub fn print(string:&'static str) {
    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") string.as_ptr(),
            in("rdx") string.len(),
            options(nostack)
        )
    }
}