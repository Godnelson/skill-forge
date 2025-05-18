use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
pub struct Role {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
pub struct RoleToCreate {
    pub name: String
}