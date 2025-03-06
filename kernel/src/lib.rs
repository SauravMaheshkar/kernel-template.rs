#![no_std]

pub mod io;
pub mod logger;

pub fn init() {
    logger::init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
