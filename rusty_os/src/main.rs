#![no_std] // disable linking into the standard library

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
    }
}


fn main() {

}
