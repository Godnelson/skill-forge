use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub bio: String,
    pub pfp: String,
    pub cv: String,
    pub is_banned: bool,
}

pub struct UserToCreate {
    pub name: String,
    pub bio: String,
    pub pfp: String,
    pub cv: String
}