#![no_std]

use bootloader_api::BootInfo;

pub mod logger;

/// Initializes the kernel
///
/// This is where you should add other initialization functions such as memory, interrupts, etc.
///
/// # Arguments
/// * `framework_info` - ['bootloader_api::BootInfo'](https://docs.rs/bootloader_api/latest/bootloader_api/info/struct.BootInfo.html) - The boot information provided by `bootloader`
/// * `frame_buffer_logger_status` - [`bool`] - Whether to enable the frame buffer logger
/// * `serial_logger_status` - [`bool`] - Whether to enable the serial logger
pub fn init(
    framework_info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    logger::init(
        framework_info,
        frame_buffer_logger_status,
        serial_logger_status,
    );
}

/// Sends the CPU into an infinite loop that waits for the next interrupt
///
/// Internally calls [`x86_64::instructions::hlt()`]
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
