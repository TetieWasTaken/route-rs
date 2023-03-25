use std::{error::Error};

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    name: String,
    start_lat: f64,
    stop_lat: f64,
    start_lon: f64,
    stop_lon: f64,
    speed_limit: f64,
    lane_count: f64,
    road_type: String,
}

pub fn load_data() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("src/sample/roads.csv")?;
    let records: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}