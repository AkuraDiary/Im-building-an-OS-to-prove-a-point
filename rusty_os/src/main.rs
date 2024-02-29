#![no_std] // disable linking into the standard library
#![no_main] // tell the rust compiler to not use the normal entry point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn test_panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     crate::test_panic_handler(info)
// }

// TEST RUNNER
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Initiating Test Runner");
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success)
}

// TEST CASESS
#[test_case]
pub fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/*
By using the #[no_mangle] attribute,
we disable name mangling to ensure that the Rust compiler
really outputs a function with the name _start.
Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE
*/
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("I'm Building An OS To Prove A Point");
    println!("ayam");

    #[cfg(test)]
    test_main();

    println!("bebek");
    loop {}
}
