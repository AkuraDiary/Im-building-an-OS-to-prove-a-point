#![no_std] // disable linking into the standard library
#![no_main] // tell the rust compiler to not use the normal entry point

use core::panic::PanicInfo;
mod vga_buffer;


/*
By using the #[no_mangle] attribute,
we disable name mangling to ensure that the Rust compiler
really outputs a function with the name _start.
Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE
*/
// here we're defining our own entry point
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn _start() -> !{
    vga_buffer::print_something();
    loop {
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
    }
}
