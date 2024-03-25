use postgres::Error;

use crate::parking::vacancies::repo::VacanciesRepo;
use crate::parking::vacancies::vacancy::{ParkingVacancy, VacancyType};
use crate::regions::service::RegionsService;
use crate::regions::region::{Region};
use crate::app_state::AppState;

struct VacanciesService {
    repo: VacanciesRepo,
    regions_service: RegionsService
}

impl VacanciesService {
    pub fn new(state: AppState) -> Self {
        Self {
            repo: VacanciesRepo::new(state.storage),
            regions_service: state.regions_service
        }
    }


    // available vacancies are going to be the vacancies whose distance to the driver is less than or equal to radius
    // and whose status == FREE and also has the same type as the driver's vehicle
    pub fn get_available_vacancies(&self, driver_latitude:f32, driver_longitude: f32, t: VacancyType) -> Vec<ParkingVacancy> {
        let vacancies: Vec<ParkingVacancy> = Vec::new();
        let vacancies_result = self.repo.get_close_vacancies(500.0, driver_longitude, driver_latitude, super::vacancy::VacancyStatus::FREE, t);
        match vacancies_result {
            Ok(rows) => {
                for row in rows.iter() {
                    let maybe_vac_region = self.regions_service.get_region(row.get("region"));
                    if let Some(vac_region) = maybe_vac_region {
                        vacancies.push(ParkingVacancy {
                            id: row.get("id"),
                            region: vac_region,
                            status: super::vacancy::VacancyStatus::from(row.get("status")),
                            t: super::vacancy::VacancyType::from(row.get("t") as i8)
                        });
                    }
                }
                vacancies
            },
            Err(err) => {} 
        }
    }

}
