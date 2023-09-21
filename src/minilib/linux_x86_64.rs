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
        asm!("syscall",
             in("rax") 1,
             in("rdi") 1,
             in("rsi") string.as_ptr(),
             in("rdx") string.len(),
             options(nostack)
        )
    }
}