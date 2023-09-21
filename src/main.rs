#![no_std]
#![no_main]
#![feature(naked_functions)]

mod minilib;
use minilib::*;
use core::arch::asm;

#[no_mangle]
pub unsafe fn main(args: &[*const u8]) {
    print("Hello, World!\nArguments:\n");

    for &arg in args {
        print(" - ");
        write(arg, strlen(arg));
        print("\n");
    }

    exit(0);
}

#[panic_handler]
unsafe fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    exit(255)
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    #[cfg(all(target_arch="x86_64", target_os="linux"))]
    asm!("mov rdi, rsp", "call get_args", options(noreturn))
}

#[no_mangle]
pub unsafe fn get_args(stack_top: *const u8) {
    let argc = *(stack_top as *const u64);
    let argv = stack_top.add(8) as *const *const u8;

    use core::slice::from_raw_parts as mkslice;
    let args = mkslice(argv, argc as usize);    

    main(args)
}
