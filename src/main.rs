#![no_std]
#![no_main]
#![feature(naked_functions)]

mod minilib;
use core::slice::from_raw_parts as mkslice;
use minilib::*;

#[no_mangle]
pub unsafe fn main(args: &[*const u8]) -> ! {
    print_str(b"Hello");

    if args.len() > 1 {
        for i in 1..args.len() {
            print_str(b" ");
            sys_write(args[i], strlen(args[i]));
        }
    } else {
        print_str(b" World");
    }

    print_str(b"!\n");

    sys_exit(0)
}

#[no_mangle]
unsafe fn get_args(stack_top: *const u8) {
    let argc: usize = *(stack_top as *const usize);
    let argv: *const *const u8 = stack_top.add(8) as *const *const u8;
    let args: &[*const u8] = mkslice(argv, argc as usize);
    main(args)
}

#[panic_handler]
unsafe fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    sys_exit(255)
}
