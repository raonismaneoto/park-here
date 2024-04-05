use crate::{
    database::storage::Storage,
    AppError::{default::DefaultAppError, error::AppError},
};

use super::{
    credentials::{self, credentials::Credentials},
    users::user::User,
};

#[derive(Clone)]
pub struct AuthRepo {
    storage: Storage,
}

impl AuthRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }

    pub async fn save_credentials(&self, credentials: Credentials) -> bool {
        let cmd = String::from(
            "INSERT INTO
                Credentials 
                    (passwd, username, user_id)
                VALUES
                    ($1, $2, $3);",
        );

        self.storage
            .exec(
                cmd,
                &[
                    &credentials.passwd,
                    &credentials.username,
                    &credentials.user_id,
                ],
            )
            .await
    }

    pub async fn save_user(&self, user: User) -> bool {
        let cmd = String::from(
            "INSERT INTO
                app_user 
                    (id, uname)
                VALUES
                    ($1, $2);",
        );

        self.storage.exec(cmd, &[&user.id, &user.name]).await
    }

    pub async fn get_user_credentials(
        &self,
        user_id: String,
    ) -> Result<Credentials, Box<dyn AppError>> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                Credentials 
            WHERE
                user_id = $1;",
        );

        match self.storage.query(cmd, &[&user_id]).await {
            Ok(rows) => {
                if rows.len() == 0 {
                    Err(Box::new(DefaultAppError {
                        message: Some(String::from("unable to locate user credentials")),
                        status_code: 404,
                    }))
                } else {
                    let creds_row = &rows[0];
                    Ok(Credentials {
                        passwd: creds_row.get("passwd"),
                        user_id: creds_row.get("user_id"),
                        username: creds_row.get("username"),
                    })
                }
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(err.to_string()),
                status_code: 500,
            })),
        }
    }

    pub async fn get_user(&self, user_id: String) -> Result<User, Box<dyn AppError>> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                app_user 
            WHERE
                user_id = $1;",
        );

        match self.storage.query(cmd, &[&user_id]).await {
            Ok(rows) => {
                if rows.len() == 0 {
                    Err(Box::new(DefaultAppError {
                        message: Some(String::from("unable to locate user entity")),
                        status_code: 404,
                    }))
                } else {
                    let row = &rows[0];
                    Ok(User {
                        id: row.get("id"),
                        name: row.get("uname"),
                    })
                }
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(err.to_string()),
                status_code: 500,
            })),
        }
    }
}
