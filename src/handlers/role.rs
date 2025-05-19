use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::models::role::{Role, RoleToCreate};
use crate::repos::role::read_roles as repo_read_roles;
use crate::repos::role::read_role_by_id as repo_read_role_by_id;
use crate::repos::role::create_role as repo_create_role;
use crate::repos::role::update_role as repo_update_role;
use crate::repos::role::delete_role as repo_delete_role;

#[utoipa::path(
    get,
    path = "/role",
    responses(
        (status = 200, description = "Get roles success", body = Vec<Role>)
    )
)]
pub async fn read_roles(State(pool): State<Pool<Postgres>>) -> impl IntoResponse{
    let roles = repo_read_roles(pool).await;
    match roles {
        Ok(roles) => (StatusCode::OK, serde_json::to_string(&roles).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}

#[utoipa::path(
    get,
    path = "/role/{id}",
    responses(
        (status = 200, description = "Get role success", body = Role)
    )
)]
pub async fn read_role_by_id(State(pool): State<Pool<Postgres>>, Path(id): Path<String>) -> impl IntoResponse{
    let role = repo_read_role_by_id(pool, id).await;
    match role {
        Ok(role) => (StatusCode::OK, serde_json::to_string(&role).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}

#[utoipa::path(
    post,
    path = "/role/create",
    responses(
        (status = 201, description = "Create role success", body = Role)
    )
)]
pub async fn create_role(State(pool): State<Pool<Postgres>>, Json(role_to_create): Json<RoleToCreate>) -> impl IntoResponse{
    let role = Role {
        id: Uuid::new_v4().to_string(),
        name: role_to_create.name,
    };
    let result = repo_create_role(pool, &role).await;
    
    match result { 
        Ok(_) => (StatusCode::CREATED, serde_json::to_string_pretty(&role).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}

#[utoipa::path(
    post,
    path = "/role/update",
    responses(
        (status = 200, description = "Update role success", body = Role)
    )
)]
pub async fn update_role(State(pool): State<Pool<Postgres>>, Json(role): Json<Role>) -> impl IntoResponse{
    let result = repo_update_role(pool, &role).await;
    
    match result {
        Ok(_) => (StatusCode::CREATED, serde_json::to_string_pretty(&role).unwrap()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}

#[utoipa::path(
    post,
    path = "/role/delete",
    responses(
        (status = 204, description = "Delete role success", body = Role)
    )
)]
pub async fn delete_role(State(pool): State<Pool<Postgres>>, Path(id): Path<String>) -> impl IntoResponse{
    let result = repo_delete_role(pool, id).await;

    match result {
        Ok(_) => (StatusCode::NO_CONTENT, "".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string())
    }
}