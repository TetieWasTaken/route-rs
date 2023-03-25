use std::{ error::Error };

mod loaddata;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    let (roads, intersections) = loaddata::load_data()?;
    window::init(roads, intersections);
    Ok(())
}