#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os_test::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Hello {}!", "World");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
