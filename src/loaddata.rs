use std::{error::Error};

#[derive(Debug, serde::Deserialize)]
pub struct Road {
    name: String,
    start_lat: f64,
    stop_lat: f64,
    start_lon: f64,
    stop_lon: f64,
    speed_limit: f64,
    lane_count: f64,
    road_type: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Intersection {
    pub id: i32,
    pub lat: f64,
    pub lon: f64,
    pub traffic_lights: bool,
}

pub fn load_roads() -> Result<Vec<Road>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("src/sample/roads.csv")?;
    let records: Vec<Road> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

pub fn load_intersections() -> Result<Vec<Intersection>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("src/sample/intersections.csv")?;
    let records: Vec<Intersection> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

pub fn load_data() -> Result<(Vec<Road>, Vec<Intersection>), Box<dyn Error>> {
    let roads = load_roads()?;
    let intersections = load_intersections()?;
    Ok((roads, intersections))
}