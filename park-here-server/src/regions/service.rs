use crate::app_state::AppState;
use crate::regions::region::Region;
use crate::regions::repo::RegionRepo;

pub struct RegionsService {
    repo: RegionRepo
}

impl RegionsService {
    pub fn new(state: AppState) -> Self {
        Self{
            repo: RegionRepo::new(state.storage)
        }
    }

    pub fn get_region(&self, id: String) -> Option<Region> {
        let region_query_result = self.repo.get_region(id);
        match region_query_result {
            Ok(regions) => {
                Option::Some(
                    Region {
                        latitude: regions.first()?.get("latitude"),
                        longitude: regions.first()?.get("longitude"),
                        id: id
                    }
                )
            },
            Err(err) => Option::None
        }
    }
}