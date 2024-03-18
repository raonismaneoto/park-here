use crate::parking::vacancies::repo::VacanciesRepo;
use crate::parking::vacancies::vacancy::{ParkingVacancy, VacancyType};
use crate::regions::region::{Region};

// available vacancies are going to be the vacancies whose distance to the driver is less than or equal to radius
// and whose status == FREE and also has the same type as the driver's vehicle
pub fn get_available_vacancies(driver_latitude:f32, driver_longitude: f32, t: VacancyType) -> Vec<ParkingVacancy>{
    let vacancy1 = ParkingVacancy{
        id: String::from("1"),
        region: Region{
            latitude: -11.032,
            longitude: -30.50,
            id: String::from("")
        },
        status: crate::parking::vacancies::vacancy::VacancyStatus::FREE,
        t: VacancyType::MOTORCYCLE
    };

    let vacancy2 = ParkingVacancy{
        id: String::from("2"),
        region: Region{
            latitude: 37.4220936,
            longitude: -122.083922,
            id: String::from("")
        },
        status: crate::parking::vacancies::vacancy::VacancyStatus::FREE,
        t: VacancyType::CAR
    };

    vec![vacancy1, vacancy2]
}