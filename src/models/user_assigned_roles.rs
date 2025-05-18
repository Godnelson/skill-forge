use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
pub struct UserAssignedRoles {
    user_id: String,
    role_id: String,
}