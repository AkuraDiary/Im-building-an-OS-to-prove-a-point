#![no_std] // disable linking into the standard library
#![no_main] // tell the rust compiler to not use the normal entry point

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
    }
}


// here we're defining our own entry point
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop {

    }
}
