use crate::database::storage::Storage;
use postgres::{Error, Row};

#[derive(Clone)]
pub struct RegionRepo {
    storage: Storage,
}

impl RegionRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }

    pub async fn get_region(&self, id: String) -> Result<Vec<Row>, Error> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                Region 
            WHERE
                id = $1;",
        );

        self.storage.query(cmd, &[&id]).await
    }

    pub async fn save_region(&self, id: String, latitude: f64, longitude: f64) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Region 
                    (id, latitude, longitude)
                VALUES
                    ($1, $2, $3);",
        );

        self.storage.exec(cmd, &[&id, &latitude, &longitude]).await
    }
}
