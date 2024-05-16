#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod uart;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    uart::init();
    uart::put_s("Hello, world!/n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
