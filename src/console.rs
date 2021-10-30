use crate::drivers;

use core::fmt::{self, Write};

// TODO ACCESS MUTEX

static mut CONSOLE: Option<drivers::serial::SerialConsole> = None;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::console::_print(format_args!("\r\n"))
    };

    ($($arg:tt)*) => {
        $crate::console::_print(format_args!("{}\n\r", format_args!($($arg)*)))
    };
}

pub fn init_serial() {
    unsafe {
        CONSOLE = drivers::serial::SerialConsole::take();
        let serial = CONSOLE.as_ref().unwrap();

        serial.enable();
    };
}

pub fn _print(args: fmt::Arguments) {
    unsafe { CONSOLE.as_mut().unwrap().write_fmt(args).unwrap() }
}
