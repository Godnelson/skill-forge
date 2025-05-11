use crate::models::user::{User, UserDTO, UserToCreate};
use crate::repos::user::create_user as repo_create_user;
use crate::repos::user::delete_user as repo_delete_user;
use crate::repos::user::read_user_by_id as repo_read_users_by_id;
use crate::repos::user::read_users as repo_read_users;
use crate::repos::user::update_user as repo_update_user;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get users success", body = Vec<User>)
    )
)]
pub async fn read_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let users = repo_read_users(pool).await;
    match users {
        Ok(users) => (StatusCode::OK, serde_json::to_string_pretty(&users).unwrap()).into_response(),
        Err(err) => (StatusCode::OK, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    responses(
        (status = 200, description = "Get users success", body = User),
        (status = 404, description = "Not found", body = String)
    )
)]
pub async fn read_users_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = repo_read_users_by_id(pool, id)
        .await
        .unwrap()
        .ok_or("User not found");
    match user {
        Ok(user) => (StatusCode::OK, serde_json::to_string_pretty(&user).unwrap()).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/user/create",
    responses(
        (status = 201, description = "Create user success", body = User),
        (status = 422, description = "Unprocessable Entity", body = String),
        (status = 400, description = "Bad request", body = String)
    )
)]
pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<UserToCreate>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: user.name,
        email: user.email,
        password: password_hash,
        bio: user.bio,
        pfp: user.pfp,
        cv: user.cv,
        is_banned: false,
    };
    let result = repo_create_user(pool, user.clone()).await;
    match result {
        Ok(_) => (
            StatusCode::CREATED,
            serde_json::to_string_pretty(&user).unwrap(),
        )
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/user/update",
    responses(
        (status = 200, description = "Update user success", body = User),
        (status = 400, description = "Bad request", body = String)
    )
)]
pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<UserDTO>,
) -> impl IntoResponse {
    let result = repo_update_user(pool, user.clone()).await;
    match result {
        Ok(_) => (StatusCode::OK, serde_json::to_string_pretty(&user).unwrap()).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/user/delete/{id}",
    responses(
        (status = 204, description = "Delete user success", body = User),
        (status = 400, description = "Bad request", body = String)
    )
)]
pub async fn delete_user(State(pool): State<Pool<Postgres>>, id: String) -> impl IntoResponse {
    let result = repo_delete_user(pool, id).await;
    match result {
        Ok(_) => (
            StatusCode::OK,
            json!({ "message": "User deleted successfully" }).to_string(),
        )
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
