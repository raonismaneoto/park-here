use super::users::user::User;

pub struct AuthService {}

impl AuthService {
    fn new() -> Self {
        Self {}
    }

    fn login(&self) -> User {
        User {
            id: String::from(""),
            vehicles_ids: vec![String::from("")],
            name: String::from("")
        }
    }
}