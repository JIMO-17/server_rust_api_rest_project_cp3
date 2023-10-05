use serde::{Deserialize};

#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}