use std::{error::Error};

extern crate piston_window;

mod loaddata;

fn main() -> Result<(), Box<dyn Error>> {
    loaddata::main()?;
    Ok(())
}
