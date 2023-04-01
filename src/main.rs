use std::env::current_dir;
use std::error::Error;

mod constants;
mod helpers;
mod managers;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    if !current_dir()?.to_str().unwrap().ends_with("src") {
        panic!("[WRONG_DIR] Please run this program from the src directory");
    }

    // Todo: make logger global
    let logger = helpers::logger::Logger::new(helpers::logger::LogLevel::Trace);

    logger.info("(main) run window");
    window::init(&logger);
    Ok(())
}
