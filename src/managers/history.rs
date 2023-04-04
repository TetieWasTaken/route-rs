use crate::managers::intersection::Intersection;
use crate::managers::road::Road;

#[derive(Debug)]
pub enum HistoryEntryData {
    Road(Road),
    Intersection(Intersection),
}

#[derive(Debug)]
pub enum HistoryEntryType {
    Create,
    Update,
    Destroy,
}

#[derive(Debug)]
pub enum Manager {
    Road,
    Intersection,
}

#[derive(Debug)]
pub struct HistoryEntry {
    pub data: HistoryEntryData,
    pub entry_type: HistoryEntryType,
    pub manager: Manager,
}

#[derive(Debug)]
pub struct HistoryManager {
    history: Vec<HistoryEntry>,
}

impl HistoryManager {
    pub fn new() -> HistoryManager {
        HistoryManager {
            history: Vec::new(),
        }
    }

    pub fn create(&mut self, entry: HistoryEntry) {
        println!("Creating history entry: {:?}", entry);

        self.history.push(entry);

        println!("History: {:?}", self.history);

        if self.history.len() > 15 {
            self.history.remove(0);
        }
    }

    pub fn undo(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        println!("Undoing: {:?}", self.history.last().unwrap());

        let entry = self.history.pop().unwrap();

        println!("Entry: {:?}", entry);

        match entry.manager {
            Manager::Road => match entry.entry_type {
                HistoryEntryType::Create => {
                    println!("CREATE TYPE");

                    let road = match entry.data {
                        HistoryEntryData::Road(road) => road,
                        _ => panic!("[HISTORY_MANAGER] Wrong entry data"),
                    };

                    println!("Road: {:?}", road);

                    crate::get_road_manager()
                        .write()
                        .unwrap()
                        .destroy(road._id.unwrap());
                }
                HistoryEntryType::Update => {
                    // TODO
                }

                HistoryEntryType::Destroy => {
                    let road = match entry.data {
                        HistoryEntryData::Road(road) => road,
                        _ => panic!("[HISTORY_MANAGER] Wrong entry data"),
                    };

                    crate::get_road_manager().write().unwrap().create(road);
                }
                _ => {
                    panic!("[HISTORY_MANAGER] Wrong entry type");
                }
            },

            Manager::Intersection => match entry.entry_type {
                HistoryEntryType::Create => {
                    let intersection = match entry.data {
                        HistoryEntryData::Intersection(intersection) => intersection,
                        _ => panic!("[HISTORY_MANAGER] Wrong entry data"),
                    };

                    crate::get_intersection_manager()
                        .lock()
                        .unwrap()
                        .destroy(intersection._id.unwrap());
                }
                HistoryEntryType::Update => {
                    // TODO
                }

                HistoryEntryType::Destroy => {
                    let intersection = match entry.data {
                        HistoryEntryData::Intersection(intersection) => intersection,
                        _ => panic!("[HISTORY_MANAGER] Wrong entry data"),
                    };

                    crate::get_intersection_manager()
                        .lock()
                        .unwrap()
                        .create(intersection);
                }
                _ => {
                    panic!("[HISTORY_MANAGER] Wrong entry type");
                }
            },
        }
    }
}
