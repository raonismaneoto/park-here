use crate::models::{ParkingVacancy, Region};

pub fn get_available_vacancies() -> Vec<ParkingVacancy>{
    let vacancy = ParkingVacancy{
        id: String::from("1"),
        region: Region{
            latitude: -11.032,
            longitude: -30.50
        },
        status: crate::models::VacancyStatus::FREE
    };
    vec![vacancy]
}