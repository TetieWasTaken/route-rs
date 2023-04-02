use lazy_static::lazy_static;
use std::env::current_dir;
use std::error::Error;

mod constants;
mod helpers;
mod managers;
mod window;

lazy_static! {
    pub static ref LOGGER: helpers::logger::Logger =
        helpers::logger::Logger::new(helpers::logger::LogLevel::Trace);
}

/// Returns the logger
pub fn get_logger() -> &'static helpers::logger::Logger {
    &LOGGER
}

fn main() -> Result<(), Box<dyn Error>> {
    if !current_dir()?.to_str().unwrap().ends_with("src") {
        panic!("[WRONG_DIR] Please run this program from the src directory");
    }

    let logger = get_logger();

    logger.info("(main) run window");
    window::init();
    Ok(())
}
