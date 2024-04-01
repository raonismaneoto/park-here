use tokio_postgres::Error;

use crate::database::storage::Storage;

use super::{credentials::{self, credentials::Credentials}, users::user::User};

#[derive(Clone)]
pub struct AuthRepo {
    storage: Storage,
}

impl AuthRepo {
    pub async fn save_credentials(&self, credentials: Credentials) -> bool {

    }

    pub async fn save_user(&self, user: User) -> bool {

    }

    pub async fn get_user_credentials(&self, user_id: String) -> Result<Credentials, Error> {

    }

    pub async fn get_user(&self, user_id: String) -> Result<User, Error> {

    }
}