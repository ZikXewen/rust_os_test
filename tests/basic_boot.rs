#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os_test::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rust_os_test::test_panic(info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println");
}
