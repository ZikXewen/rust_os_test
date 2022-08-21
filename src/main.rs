#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
mod test;
mod vga_buffer;

#[cfg(not(test))]
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
