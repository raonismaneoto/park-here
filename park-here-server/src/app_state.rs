use crate::{database::storage::Storage, regions::service::RegionsService};

pub struct AppState {
    pub storage: Storage,
    pub regions_service: RegionsService
}