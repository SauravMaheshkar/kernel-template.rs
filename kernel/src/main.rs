#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

/// The entry point of the kernel
///
/// This function is called by the boot code in `boot.s`
#[no_mangle]
fn kernel_main(_info: &'static mut BootInfo) -> ! {
    // let framebuffer = info.framebuffer.as_mut();

    kernel::init();

    kernel::hlt_loop();
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `_info` - The panic information
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::hlt_loop();
}
