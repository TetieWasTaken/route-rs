use std::env::current_dir;
use std::error::Error;

mod helpers;
mod loaddata;
mod managers;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    if !current_dir()?.to_str().unwrap().ends_with("src") {
        panic!("[WRONG_DIR] Please run this program from the src directory");
    }

    let (roads, intersections) = loaddata::load_data()?;
    window::init(roads, intersections);
    Ok(())
}
