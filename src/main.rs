#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(vga_buffer::WRITER.lock(), "Hello\n{}", "Test").ok();
    loop {}
}
