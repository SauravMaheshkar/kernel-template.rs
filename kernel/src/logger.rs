use bootloader_boot_config::LevelFilter;

use bootloader_api::BootInfo;

use bootloader_x86_64_common::init_logger;

pub fn init(info: &'static mut BootInfo) {
    let framebuffer = info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    init_logger(buffer, info, LevelFilter::Info, true, true);

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
