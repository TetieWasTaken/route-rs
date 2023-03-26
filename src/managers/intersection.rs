#[derive(Debug, serde::Deserialize)]
pub struct Intersection {
    pub id: i32,
    pub lat: f64,
    pub lon: f64,
    pub traffic_lights: bool,
}