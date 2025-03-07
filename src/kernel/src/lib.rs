#![no_std]

use bootloader_api::BootInfo;

pub mod logger;

pub fn init(framework_info: &'static mut BootInfo) {
    logger::init(framework_info);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
