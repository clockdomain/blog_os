// main.rs
#![no_std]   // We don't want to link to the standard library.
#![no_main]  // We want to override crt0 entry point
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic (_info : &PanicInfo) -> ! {
    loop {}
}
