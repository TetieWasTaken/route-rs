use std::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct Road {
    pub name: String,
    pub start_lat: f64,
    pub stop_lat: f64,
    pub start_lon: f64,
    pub stop_lon: f64,
    pub speed_limit: f64,
    pub lane_count: f64,
    pub road_type: String,
}

impl Road {
    pub fn get_points(&self) -> [f64; 4] {
        [self.start_lat, self.start_lon, self.stop_lat, self.stop_lon]
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Intersection {
    pub id: i32,
    pub lat: f64,
    pub lon: f64,
    pub traffic_lights: bool,
}

pub fn load_roads() -> Result<Vec<Road>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("sample/roads.csv")?;
    let records: Vec<Road> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

pub fn load_intersections() -> Result<Vec<Intersection>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("sample/intersections.csv")?;
    let records: Vec<Intersection> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

pub fn load_data() -> Result<(Vec<Road>, Vec<Intersection>), Box<dyn Error>> {
    let roads = load_roads()?;
    let intersections = load_intersections()?;
    Ok((roads, intersections))
}
