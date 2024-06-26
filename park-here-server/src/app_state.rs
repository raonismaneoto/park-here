use crate::{
    auth::service::AuthService, database::storage::Storage,
    parking::vacancies::service::VacanciesService, regions::service::RegionsService,
};

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub regions_service: RegionsService,
    pub vacancies_service: VacanciesService,
    pub auth_service: AuthService,
}

impl AppState {
    pub fn build() -> Self {
        let storage = Storage::new(
            String::from("localhost"),
            String::from("postgres"),
            String::from("postgres"),
            String::from("park-here-123"),
        );
        let regions_service = RegionsService::new(storage.clone());
        let vacancies_service = VacanciesService::new(regions_service.clone(), storage.clone());
        let auth_service = AuthService::new(storage.clone());

        Self {
            storage: storage.clone(),
            regions_service: regions_service.clone(),
            vacancies_service: vacancies_service.clone(),
            auth_service: auth_service.clone(),
        }
    }
}
