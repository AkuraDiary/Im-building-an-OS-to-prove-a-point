#![no_std] // disable linking into the standard library
#![no_main] // tell the rust compiler to not use the normal entry point

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
    }
}


// here we're defining our own entry point

/*
By using the #[no_mangle] attribute,
we disable name mangling to ensure that the Rust compiler
really outputs a function with the name _start.
Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE
*/

static HELLO: &[u8] = b"Hello World";
#[no_mangle]
pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb800 as *mut u8;
    for(i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {
    }
}
