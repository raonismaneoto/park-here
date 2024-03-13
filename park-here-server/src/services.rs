use crate::models::{ParkingVacancy, Region};

pub fn get_available_vacancies() -> Vec<ParkingVacancy>{
    let vacancy1 = ParkingVacancy{
        id: String::from("1"),
        region: Region{
            latitude: -11.032,
            longitude: -30.50
        },
        status: crate::models::VacancyStatus::FREE
    };

    let vacancy2 = ParkingVacancy{
        id: String::from("2"),
        region: Region{
            latitude: 37.4220936,
            longitude: -122.083922
        },
        status: crate::models::VacancyStatus::FREE
    };

    vec![vacancy1, vacancy2]
}