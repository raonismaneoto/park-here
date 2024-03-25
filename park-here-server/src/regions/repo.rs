use postgres::{Row, Error};
use crate::database::storage::{Storage};

pub struct RegionRepo {
    storage: Storage
}

impl RegionRepo {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage: storage
        }
    }

    pub fn get_region(&self, id: String) -> Result<Vec<Row>, Error> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                Region 
            WHERE
                id = $1;"
        );

        self.storage.query(cmd, &[&id])
    }

    pub fn save_region(&self, id: String, latitude: f32, longitude: f32) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Region 
                    (id, latitude, longitude)
                VALUES
                    ($1, $2, $3);"
        );

        self.storage.exec(cmd, &[&id, &latitude, &longitude])
    }
}