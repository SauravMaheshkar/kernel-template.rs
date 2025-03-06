#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};

use kernel::println;

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
    kernel::info!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    kernel::hlt_loop();
}

/// Simple panic handler that loops forever
///
/// # Arguments
/// * `_info` - The panic information
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC]: {}\n", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
