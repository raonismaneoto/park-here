use crate::parking::vacancies::vacancy::{ParkingVacancy, VacancyType};
use crate::regions::region::{Region};

pub fn get_available_vacancies() -> Vec<ParkingVacancy>{
    let vacancy1 = ParkingVacancy{
        id: String::from("1"),
        region: Region{
            latitude: -11.032,
            longitude: -30.50
        },
        status: crate::parking::vacancies::vacancy::VacancyStatus::FREE,
        t: VacancyType::MOTORCYCLE
    };

    let vacancy2 = ParkingVacancy{
        id: String::from("2"),
        region: Region{
            latitude: 37.4220936,
            longitude: -122.083922
        },
        status: crate::parking::vacancies::vacancy::VacancyStatus::FREE,
        t: VacancyType::CAR
    };

    vec![vacancy1, vacancy2]
}