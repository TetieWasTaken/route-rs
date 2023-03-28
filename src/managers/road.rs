use std::vec;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Road {
    pub id: Option<i32>,
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

#[derive(Debug)]
pub struct RoadManager {
    pub cache: Option<Vec<Road>>,
}

impl RoadManager {
    fn _add(&mut self, road: Road) {
        if let Some(cache) = &mut self.cache {
            cache.push(road);
        }
    }

    fn _remove(&mut self, id: i32) {
        if let Some(cache) = &mut self.cache {
            let mut index = 0;
            for road in &mut *cache {
                if road.id == Some(id) {
                    cache.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }

    pub fn resolve(&mut self, id: i32) -> Option<&Road> {
        if let Some(cache) = &self.cache {
            for road in cache {
                if road.id == Some(id) {
                    return Some(road);
                }
            }
        }
        None
    }

    pub fn create(&mut self, road: Road) {
        let mut id = 1;

        while self
            .cache
            .as_ref()
            .unwrap()
            .iter()
            .any(|r| r.id == Some(id))
        {
            id += 1;
        }

        let road = Road {
            id: Some(id),
            ..road
        };

        self._add(road);
    }

    pub fn destroy(&mut self, id: i32) {
        self._remove(id);
    }

    pub fn store(&self) {
        let mut wtr = csv::Writer::from_path("data/roads.csv").unwrap();

        for road in self.cache.as_ref().unwrap() {
            wtr.serialize(road).unwrap();
        }

        wtr.flush().unwrap();
    }

    pub fn load(&mut self) {
        let mut rdr = csv::Reader::from_path("data/roads.csv").unwrap();

        self.cache = Some(Vec::<Road>::new());

        for result in rdr.deserialize() {
            let road: Road = result.unwrap();
            self._add(road);
        }
    }

    pub fn reset(&mut self) {
        self.cache = Some(Vec::<Road>::new());
    }
}
