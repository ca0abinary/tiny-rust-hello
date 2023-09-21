#![no_std]
#![no_main]

mod minilib;
use minilib::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello, world!\n");
    exit(0)
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    exit(-1)
}
