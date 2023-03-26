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
