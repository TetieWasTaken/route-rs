use lazy_static::lazy_static;
use std::env::current_dir;
use std::error::Error;
use std::sync::{Mutex, RwLock};

mod constants;
mod helpers;
mod managers;
mod window;

lazy_static! {
    pub static ref LOGGER: helpers::logger::Logger =
        helpers::logger::Logger::new(helpers::logger::LogLevel::Trace);
    pub static ref HISTORY_MANAGER: Mutex<managers::history::HistoryManager> =
        Mutex::new(managers::history::HistoryManager::new());
    pub static ref ROAD_MANAGER: RwLock<managers::road::RoadManager> =
        RwLock::new(managers::road::RoadManager::new());
    pub static ref INTERSECTION_MANAGER: Mutex<managers::intersection::IntersectionManager> =
        Mutex::new(managers::intersection::IntersectionManager::new());
}

/// Returns the logger
pub fn get_logger() -> &'static helpers::logger::Logger {
    &LOGGER
}

/// Returns the history manager
pub fn get_history_manager() -> &'static Mutex<managers::history::HistoryManager> {
    &HISTORY_MANAGER
}

/// Returns the road manager
pub fn get_road_manager() -> &'static RwLock<managers::road::RoadManager> {
    &ROAD_MANAGER
}

/// Returns the intersection manager
pub fn get_intersection_manager() -> &'static Mutex<managers::intersection::IntersectionManager> {
    &INTERSECTION_MANAGER
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
