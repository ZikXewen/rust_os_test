use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::test::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

pub trait TestFn {
    fn run(&self);
}
impl<T: Fn()> TestFn for T {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn TestFn]) {
    serial_println!("Running {} tests...", tests.len());
    tests.iter().for_each(|x| x.run());
    exit_qemu(QemuExitCode::Success);
}

#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
