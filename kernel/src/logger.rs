use spin::Once;

use crate::io::serial::SerialLogger;

pub static LOGGER: Once<SerialLogger> = Once::INIT;

pub fn init() {
    unsafe {
        LOGGER.call_once(|| SerialLogger::init());
        let logger = LOGGER.get().unwrap();
        log::set_logger(logger).expect("Logger already initialised");
    }

    log::set_max_level(log::LevelFilter::Info);
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
