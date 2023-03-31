#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Road {
    pub _id: Option<i32>,
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
    /// internal
    fn _add(&mut self, road: Road) {
        if let Some(cache) = &mut self.cache {
            cache.push(road);
        }
    }

    /// internal
    fn _remove(&mut self, id: i32) {
        if let Some(cache) = &mut self.cache {
            let mut index = 0;
            for road in &mut *cache {
                if road._id == Some(id) {
                    cache.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }

    /// Returns a road struct from the cache by id
    ///
    /// Example
    /// ```rust
    /// let road = road_manager.resolve(1).unwrap().clone(); // road with id 1 is now cloned into the road variable
    /// ```
    pub fn resolve(&mut self, id: i32) -> Option<&Road> {
        if let Some(cache) = &self.cache {
            for road in cache {
                if road._id == Some(id) {
                    return Some(road);
                }
            }
        }
        None
    }

    /// Creates a new road and adds it to the cache
    ///
    /// Example
    /// ```rust
    /// road_manager.create(Road {
    ///   _id: None,
    ///   name: "Test Road".to_string(),
    ///   start_lat: 0.0,
    ///   stop_lat: 0.0,
    ///   start_lon: 0.0,
    ///   stop_lon: 0.0,
    ///   speed_limit: 0.0,
    ///   lane_count: 0.0,
    ///   road_type: "asphalt".to_string(),
    /// }); // a new road is added to the cache, with a unique id
    /// ```
    pub fn create(&mut self, road: Road) {
        let mut id = 1;

        while self
            .cache
            .as_ref()
            .unwrap()
            .iter()
            .any(|r| r._id == Some(id))
        {
            id += 1;
        }

        let road = Road {
            _id: Some(id),
            ..road
        };

        self._add(road);
    }

    /// Removes a road from the cache by id. This will not remove the road from the roads.csv file.
    ///
    /// Example
    /// ```rust
    /// road_manager.destroy(1); // road with id 1 is now removed from the cache
    /// ```
    pub fn destroy(&mut self, id: i32) {
        self._remove(id);
    }

    /// Stores the cache to the roads.csv file. This will overwrite the file.
    ///
    /// Example
    /// ```rust
    /// road_manager.store(); // roads.csv is now overwritten by the contents of the cache
    /// ```
    pub fn store(&self) {
        let mut wtr = csv::Writer::from_path("data/roads.csv").unwrap();

        for road in self.cache.as_ref().unwrap() {
            wtr.serialize(road).unwrap();
        }

        wtr.flush().unwrap();
    }

    /// Specify a path to load in, defaults to "data/roads.csv". This will overwrite the cache.
    ///
    /// Example
    /// ```rust
    /// // loads in roads.csv from the data folder
    /// road_manager.load(); // cache is now overwritten by the contents of roads.csv
    ///
    /// // loads in a custom file
    /// road_manager.load(Some("data/custom_roads.csv")); // cache is now overwritten by the contents of custom_roads.csv
    /// ```
    pub fn load(&mut self, _path: Option<&str>) {
        let mut rdr = csv::Reader::from_path(_path.unwrap_or("data/roads.csv")).unwrap();

        self.cache = Some(Vec::<Road>::new());

        for result in rdr.deserialize() {
            let road: Road = result.unwrap();
            self._add(road);
        }
    }

    /// Resets the cache to an empty vector
    ///
    /// Example
    /// ```rust
    /// road_manager.reset(); // cache is now empty
    /// ```
    pub fn reset(&mut self) {
        self.cache = Some(Vec::<Road>::new());
    }
}
