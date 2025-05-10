use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub pfp: String,
    pub cv: String,
    pub is_banned: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserToCreate {
    pub name: String,
    pub password: String,
    pub email: String,
    pub bio: String,
    pub pfp: String,
    pub cv: String,
}
