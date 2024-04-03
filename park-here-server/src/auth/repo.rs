use tokio_postgres::Error;

use crate::database::storage::Storage;

use super::{
    credentials::{self, credentials::Credentials},
    users::user::User,
};

#[derive(Clone)]
pub struct AuthRepo {
    storage: Storage,
}

impl AuthRepo {
    pub async fn save_credentials(&self, credentials: Credentials) -> bool {
        true
    }

    pub async fn save_user(&self, user: User) -> bool {
        true
    }

    pub async fn get_user_credentials(&self, user_id: String) -> Result<Credentials, Error> {
        Ok(Credentials {
            passwd: String::from(""),
            user_id: String::from(""),
            username: String::from(""),
        })
    }

    pub async fn get_user(&self, user_id: String) -> Result<User, Error> {
        Ok(User {
            vehicles_ids: vec![],
            id: String::from(""),
            name: String::from(""),
        })
    }
}
