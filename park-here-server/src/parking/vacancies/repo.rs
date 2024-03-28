use postgres::{Error, Row};

use crate::database::storage::Storage;

use super::vacancy::{ParkingVacancy, VacancyStatus, VacancyType};

#[derive(Clone)]
pub struct VacanciesRepo {
    storage: Storage,
}

impl VacanciesRepo {
    pub fn new(storage: Storage) -> Self {
        VacanciesRepo { storage: storage }
    }

    pub fn save_vacancy(&self, vacancy: ParkingVacancy) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Parking_Vacancy 
                    (id, v_status, t, region_id)
                VALUES
                    ($1, $2, $3, $4);",
        );

        self.storage.exec(
            cmd,
            &[
                &vacancy.id,
                &i32::from(vacancy.status),
                &i32::from(vacancy.t),
                &vacancy.region.id,
            ],
        )
    }

    pub fn get_vacancy_by_id(&self, id: String) -> Result<Vec<Row>, Error> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                Parking_Vacancy 
            WHERE
                id = $1;",
        );

        self.storage.query(cmd, &[&id])
    }

    pub fn get_vacancy_by_status_and_type(
        &self,
        status: VacancyStatus,
        t: VacancyType,
    ) -> Result<Vec<Row>, Error> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                Parking_Vacancy 
            WHERE
                v_status = $1 and t = $2;",
        );

        self.storage
            .query(cmd, &[&i32::from(status), &i32::from(t)])
    }

    pub fn get_close_vacancies(
        &self,
        radius: f32,
        longitude: f32,
        latitude: f32,
        status: VacancyStatus,
        t: VacancyType,
    ) -> Result<Vec<Row>, Error> {
        let cmd = String::from(
            "
            SELECT 
                ST_DistanceSphere(
                    ST_MakePoint(Parking_Vacancy.longitude, $1),
                    ST_MakePoint(Parking_Vacancy.latitude, $2)
                ) AS distance_in_m, *
            FROM 
                Parking_Vacancy
            WHERE 
                v_status = $3
                AND t = $4
                AND distance_in_m <= $5;",
        );

        self.storage.query(
            cmd,
            &[
                &longitude,
                &latitude,
                &i32::from(status),
                &i32::from(t),
                &radius,
            ],
        )
    }
}
