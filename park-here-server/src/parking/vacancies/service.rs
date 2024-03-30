use crate::app_state::AppState;
use crate::database::storage::Storage;
use crate::parking::vacancies::repo::VacanciesRepo;
use crate::parking::vacancies::vacancy::{ParkingVacancy, VacancyType};
use crate::regions::service::RegionsService;
use crate::AppError::default::DefaultAppError;
use crate::AppError::error::AppError;

#[derive(Clone)]
pub struct VacanciesService {
    repo: VacanciesRepo,
    regions_service: RegionsService,
}

impl VacanciesService {
    pub fn new(regions_service: RegionsService, storage: Storage) -> Self {
        Self {
            repo: VacanciesRepo::new(storage),
            regions_service: regions_service,
        }
    }

    // available vacancies are going to be the vacancies whose distance to the driver is less than or equal to radius
    // and whose status == FREE and also has the same type as the driver's vehicle
    pub async fn get_available_vacancies(
        &self,
        driver_latitude: f64,
        driver_longitude: f64,
        t: VacancyType,
    ) -> Result<Vec<ParkingVacancy>, Box<dyn AppError>> {
        let mut vacancies: Vec<ParkingVacancy> = Vec::new();
        let vacancies_result = self.repo.get_close_vacancies(
            500.0,
            driver_longitude,
            driver_latitude,
            super::vacancy::VacancyStatus::FREE,
            t,
        ).await;

        match vacancies_result {
            Ok(rows) => {
                for row in rows.into_iter() {
                    let maybe_vac_region = self.regions_service.get_region(row.get("region_id")).await;
                    if let Some(vac_region) = maybe_vac_region {
                        let status: i32 = row.get("v_status");
                        let t: i32 = row.get("t");
                        vacancies.push(ParkingVacancy {
                            id: row.get("id"),
                            region: vac_region,
                            status: super::vacancy::VacancyStatus::from(status),
                            t: super::vacancy::VacancyType::from(t),
                        });
                    }
                }
                Ok(vacancies)
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(err.to_string()),
                status_code: 500,
            })),
        }
    }
}
