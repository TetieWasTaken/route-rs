use std::{error::Error};

#[derive(Debug, serde::Deserialize)]
struct Record {
    name: String,
    start_lat: f64,
    stop_lat: f64,
    start_lon: f64,
    stop_lon: f64,
    speed_limit: f64,
    lane_count: f64,
    road_type: String,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Loading data");
    let mut rdr = csv::Reader::from_path("src/sample/roads.csv")?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}