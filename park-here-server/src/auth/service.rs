use crate::{requests::payloads::Subscription, AppError::error::AppError};

use super::{credentials::{self, credentials::Credentials}, users::user::User};

pub struct AuthService {}

impl AuthService {
    fn new() -> Self {
        Self {}
    }

    pub async fn login(&self, credentials: Credentials) -> User {
        User {
            id: String::from(""),
            vehicles_ids: vec![String::from("")],
            name: String::from(""),
        }
    }

    pub async fn subscribe(&self, subscription_payload: Subscription) -> Result<User, Box<dyn AppError>> {
        let user = User {
            id : subscription_payload.id,
            name: subscription_payload.name,
            vehicles_ids: subscription_payload.vehicles_ids
        };

        let credentials = Credentials {
            passwd: subscription_payload.passwd,
            username: subscription_payload.username
        };

        Ok(user)
    }
}
