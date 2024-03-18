use crate::database::storage::Storage;

use super::vacancy::{ParkingVacancy, VacancyStatus, VacancyType};

pub struct VacanciesRepo {
    storage: Storage
}

impl VacanciesRepo {
    pub fn new(storage: Storage) -> Self {
        VacanciesRepo {
            storage: storage
        }
    }

    pub fn save_vacancy(&self, vacancy: ParkingVacancy) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Parking_Vacancy 
                    (id, status, t, region_id)
                VALUES
                    ($1, $2, $3, $4);"
        );

        self.storage.exec(cmd, &[&vacancy.id, &vacancy.status, &vacancy.t, &vacancy.region.id])
    }

    pub fn get_vacancy_by_id(&self, id: String) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Parking_Vacancy 
                    (id, status, t, region_id)
                VALUES
                    ($1, $2, $3, $4);"
        );

        self.storage.exec(cmd, &[&vacancy.id, &vacancy.status, &vacancy.t, &vacancy.region.id])
    }

    pub fn get_vacancy_by_status_and_type(&self, status: VacancyStatus, t: VacancyType) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Parking_Vacancy 
                    (id, status, t, region_id)
                VALUES
                    ($1, $2, $3, $4);"
        );

        self.storage.exec(cmd, &[&vacancy.id, &vacancy.status, &vacancy.t, &vacancy.region.id])
    }

    pub fn get_close_vacancies(&self, radius: f32, status: VacancyStatus, t: VacancyType) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Parking_Vacancy 
                    (id, status, t, region_id)
                VALUES
                    ($1, $2, $3, $4);"
        );

        self.storage.exec(cmd, &[&vacancy.id, &vacancy.status, &vacancy.t, &vacancy.region.id])
    }




}