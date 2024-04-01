use serde::Serialize;

#[derive(Serialize)]
pub struct Credentials {
    pub passwd: String,
    pub username: String,
    pub user_id: String
}