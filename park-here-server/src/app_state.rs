use crate::{
    database::storage::Storage, parking::vacancies::service::VacanciesService,
    regions::service::RegionsService,
};

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub regions_service: RegionsService,
    pub vacancies_service: VacanciesService,
}

impl AppState {
    pub fn build() -> Self {
        let storage = Storage::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        );
        let regions_service = RegionsService::new(storage.clone());
        let vacancies_service = VacanciesService::new(regions_service.clone(), storage.clone());

        Self {
            storage: storage.clone(),
            regions_service: regions_service.clone(),
            vacancies_service: vacancies_service.clone(),
        }
    }
}
