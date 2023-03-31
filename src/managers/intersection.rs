#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Intersection {
    pub _id: Option<i32>,
    pub lat: f64,
    pub lon: f64,
    pub traffic_lights: bool,
}

#[derive(Debug)]
pub struct IntersectionManager {
    pub cache: Option<Vec<Intersection>>,
}

impl IntersectionManager {
    /// internal
    fn _add(&mut self, intersection: Intersection) {
        if let Some(cache) = &mut self.cache {
            cache.push(intersection);
        }
    }

    /// internal
    fn _remove(&mut self, id: i32) {
        if let Some(cache) = &mut self.cache {
            let mut index = 0;
            for intersection in &mut *cache {
                if intersection._id == Some(id) {
                    cache.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }

    /// Returns a intersection struct from the cache by id
    ///
    /// Example
    /// ```rust
    /// let intersection = intersection_manager.resolve(1).unwrap().clone(); // intersection with id 1 is now cloned into the intersection variable
    /// ```
    pub fn resolve(&mut self, id: i32) -> Option<&Intersection> {
        if let Some(cache) = &self.cache {
            for intersection in cache {
                if intersection._id == Some(id) {
                    return Some(intersection);
                }
            }
        }
        None
    }

    /// Creates a new intersection
    ///
    /// Example
    /// ```rust
    /// intersection_manager.create(Intersection {
    ///     id: None,
    ///     lat: 0.0,
    ///     lon: 0.0,
    ///     traffic_lights: false,
    /// });
    /// ```
    pub fn create(&mut self, intersection: Intersection) {
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

        let intersection = Intersection {
            _id: Some(id),
            ..intersection
        };

        self._add(intersection);
    }

    /// Removes an intersection from the cache
    ///
    /// Example
    /// ```rust
    /// intersection_manager.destroy(1);
    /// ```
    pub fn destroy(&mut self, id: i32) {
        self._remove(id);
    }

    /// Stores the cache to the intersections.csv file. This will overwrite the file.
    ///
    /// Example
    /// ```rust
    /// intersection_manager.store(); // intersections.csv is now overwritten by the contents of the cache
    /// ```
    pub fn store(&self) {
        let mut wtr = csv::Writer::from_path("data/intersections.csv").unwrap();

        for intersection in self.cache.as_ref().unwrap() {
            wtr.serialize(intersection).unwrap();
        }

        wtr.flush().unwrap();
    }

    /// Loads intersections from a csv file
    ///
    /// Example
    /// ```rust
    /// intersection_manager.load(Some("sample/intersections.csv"));
    /// ```
    pub fn load(&mut self, _path: Option<&str>) {
        let mut rdr = csv::Reader::from_path(_path.unwrap_or("data/intersections.csv")).unwrap();

        self.cache = Some(Vec::<Intersection>::new());

        for result in rdr.deserialize() {
            let intersection: Intersection = result.unwrap();
            self._add(intersection);
        }
    }

    /// Resets the cache to an empty vector
    ///
    /// Example
    /// ```rust
    /// intersections_manager.reset(); // cache is now empty
    /// ```
    pub fn reset(&mut self) {
        self.cache = Some(Vec::<Intersection>::new());
    }
}
