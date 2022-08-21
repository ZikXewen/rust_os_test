#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_test::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    if let Some(test) = tests.first() {
        test();
        serial_println!("[did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
}

#[test_case]
fn should_fail() {
    serial_print!("Running 1 tests...\nshould_panic::should_fail...\t");
    assert_eq!(0, 1);
}
