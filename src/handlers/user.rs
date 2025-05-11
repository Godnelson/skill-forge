use crate::models::user::{User, UserToCreate};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::repos::user::create_user as repo_create_user;
use crate::repos::user::delete_user as repo_delete_user;
use crate::repos::user::read_user_by_id as repo_read_users_by_id;
use crate::repos::user::read_users as repo_read_users;
use crate::repos::user::update_user as repo_update_user;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get users success", body = Vec<User>)
    )
)]
pub async fn read_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let users: Vec<User> = repo_read_users(pool).await;
    (
        StatusCode::OK,
        serde_json::to_string_pretty(&users).unwrap(),
    )
        .into_response()
}

pub async fn read_users_by_id(State(pool): State<Pool<Postgres>>, id: String) -> impl IntoResponse {
    let user = repo_read_users_by_id(pool, id).await;
    (StatusCode::OK, serde_json::to_string_pretty(&user).unwrap()).into_response()
}

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
    let result = repo_create_user(
        pool,
        User {
            id: Uuid::new_v4().to_string(),
            name: user.name,
            email: user.email,
            password: password_hash,
            bio: user.bio,
            pfp: user.pfp,
            cv: user.cv,
            is_banned: false,
        },
    )
    .await;
    (
        StatusCode::CREATED,
        json!({"rows_affected":result.rows_affected()}).to_string(),
    )
        .into_response()
}

pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let result = repo_update_user(pool, user).await;
    (
        StatusCode::OK,
        json!({"rows_affected":result.rows_affected()}).to_string(),
    )
        .into_response()
}

pub async fn delete_user(State(pool): State<Pool<Postgres>>, id: String) -> impl IntoResponse {
    let result = repo_delete_user(pool, id).await;
    (
        StatusCode::NO_CONTENT,
        json!({"rows_affected":result.rows_affected()}).to_string(),
    )
        .into_response()
}
