/// Like the `print!` macro in the standard library, but prints to the VGA text
/// buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA
/// text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => {
        $crate::io::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println_serial {
    () => ($crate::print_serial!("\n"));
    ($fmt:expr) => ($crate::print_serial!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_serial!(
        concat!($fmt, "\n"), $($arg)*));
}
