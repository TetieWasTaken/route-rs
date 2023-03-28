use std::error::Error;

use crate::managers::intersection::Intersection;
use crate::managers::road::Road;

/// deprecated
pub fn load_roads() -> Result<Vec<Road>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("sample/roads.csv")?;
    let records: Vec<Road> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

/// deprecated
pub fn load_intersections() -> Result<Vec<Intersection>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("sample/intersections.csv")?;
    let records: Vec<Intersection> = rdr.deserialize().collect::<Result<_, _>>()?;
    Ok(records)
}

/// deprecated
pub fn load_data() -> Result<(Vec<Road>, Vec<Intersection>), Box<dyn Error>> {
    let roads = load_roads()?;
    let intersections = load_intersections()?;
    Ok((roads, intersections))
}
