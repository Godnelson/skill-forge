use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use crate::models::role::Role;
use crate::repos::role::read_roles as repo_read_roles;
use crate::repos::role::read_role_by_id as repo_read_role_by_id;

pub async fn read_roles(State(pool): State<Pool<Postgres>>) -> impl IntoResponse{
    let roles = repo_read_roles(pool).await;
    match roles {
        Ok(roles) => (StatusCode::OK, serde_json::to_string(&roles).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}

pub async fn read_role_by_id(State(pool): State<Pool<Postgres>>, Path(id): Path<String>) -> impl IntoResponse{
    let role = repo_read_role_by_id(pool, id).await;
    match role {
        Ok(role) => (StatusCode::OK, serde_json::to_string(&role).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}