//! Logger implementation that writes to the frame buffer and serial port

use bootloader_boot_config::LevelFilter;

use bootloader_api::BootInfo;

use bootloader_x86_64_common::init_logger;

/// Initializes the logger
///
/// Uses the logger from the [`bootloader-x86_64-common`](https://github.com/rust-osdev/bootloader/blob/main/common/src/logger.rs) crate.
/// This logger is compatible with the [`log`](https://github.com/rust-lang/log) crate therefore one can use the `log` macros to log messages.
///
/// # Arguments
/// * `info` - [`bootloader_api::BootInfo`] - The boot information provided by `bootloader`
/// * `frame_buffer_logger_status` - bool - Whether to enable the frame buffer logger
/// * `serial_logger_status` - bool - Whether to enable the serial logger
pub fn init(
    info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let framebuffer = info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    init_logger(
        buffer,
        info,
        LevelFilter::Info,
        frame_buffer_logger_status,
        serial_logger_status,
    );

    log::info!("Logger initialized");
    log::info!(
        r#"
        ,-~~-.___.
       / |  '     \         It was a dark and stormy night....
      (  )         0
       \_/-, ,----'
          ====           //
         /  \-'~;    /~~~(O)
        /  __/~|   /       |
      =(  _____| (_________|   W<
        "#
    );
}
