use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    database::storage::Storage,
    requests::payloads::Subscription,
    AppError::{auth::AuthError, default::DefaultAppError, error::AppError},
};

use super::{
    credentials::{self, credentials::Credentials},
    repo::{self, AuthRepo},
    users::user::User,
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Clone)]
pub struct AuthService {
    repo: repo::AuthRepo,
}

impl AuthService {
    pub fn new(storage: Storage) -> Self {
        Self {
            repo: AuthRepo::new(storage),
        }
    }

    pub async fn login(&self, credentials: Credentials) -> Result<String, Box<dyn AppError>> {
        match self
            .repo
            .get_user_credentials(credentials.user_id.clone())
            .await
        {
            Ok(creds) => {
                if creds.username == credentials.username && creds.passwd == credentials.passwd {
                    match create_jwt(credentials.user_id) {
                        Ok(jwt) => Ok(jwt),
                        Err(err) => Err(Box::new(AuthError {
                            message: err.message,
                            status_code: crate::AppError::auth::AuthErrorStatusCode::UNAUTHORIZED,
                        })),
                    }
                } else {
                    Err(Box::new(AuthError {
                        message: Some(String::from(
                            "Authentication error, either the username or the password is wrong",
                        )),
                        status_code: crate::AppError::auth::AuthErrorStatusCode::UNAUTHORIZED,
                    }))
                }
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(String::from("unable to get user credentials")),
                status_code: 500,
            })),
        }
    }

    pub async fn authorize(&self, jwt: String) -> Result<User, Box<dyn AppError>> {
        let decoded = decode::<JWTPayload>(
            &jwt,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::new(Algorithm::HS512),
        );

        match decoded {
            Ok(jwt) => self.repo.get_user(jwt.claims.sub).await,
            Err(err) => Err(Box::new(AuthError {
                message: Some(String::from("Unauthorized. Invalid Token")),
                status_code: crate::AppError::auth::AuthErrorStatusCode::UNAUTHORIZED,
            })),
        }
    }

    pub async fn subscribe(
        &self,
        subscription_payload: Subscription,
    ) -> Result<User, Box<dyn AppError>> {
        let user = User {
            id: subscription_payload.id.clone(),
            name: subscription_payload.name.clone(),
        };

        let credentials = Credentials {
            passwd: subscription_payload.passwd.clone(),
            username: subscription_payload.username.clone(),
            user_id: subscription_payload.id.clone(),
        };

        let successfully_saved_user = self.repo.save_user(user.clone()).await;

        let successfully_saved_credentials = self.repo.save_credentials(credentials).await;

        if ! (successfully_saved_credentials && successfully_saved_user) {
            Err(
                Box::new(
                    DefaultAppError {
                        message: Some(String::from("unable to save user data")),
                        status_code: 500
                    }
                )
            )
        } else {
            Ok(user)
        }
    }
}

// todo: gen this secret programatically through an ssh key gen
const JWT_SECRET: &[u8] = b"secret";

#[derive(Serialize, Deserialize)]
struct JWTPayload {
    sub: String,
    exp: usize,
}

fn create_jwt(uid: String) -> Result<String, DefaultAppError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let payload = JWTPayload {
        sub: uid.clone(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &payload, &EncodingKey::from_secret(JWT_SECRET)).map_err(|_| DefaultAppError {
        message: Some(String::from("unable to create JWT")),
        status_code: 500,
    })
}
