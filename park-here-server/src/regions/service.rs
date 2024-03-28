use crate::database::storage::Storage;
use crate::regions::region::Region;
use crate::regions::repo::RegionRepo;

#[derive(Clone)]
pub struct RegionsService {
    repo: RegionRepo,
}

impl RegionsService {
    pub fn new(storage: Storage) -> Self {
        Self {
            repo: RegionRepo::new(storage),
        }
    }

    pub async fn get_region(&self, region_id: String) -> Option<Region> {
        let region_query_result = self.repo.get_region(region_id.clone()).await;
        match region_query_result {
            Ok(regions) => Option::Some(Region {
                latitude: regions.first()?.get("latitude"),
                longitude: regions.first()?.get("longitude"),
                id: region_id.clone(),
            }),
            Err(err) => Option::None,
        }
    }
}
