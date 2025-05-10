use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use crate::models::user::{User};
use crate::repos::user::create_user as repo_create_user;

pub async fn create_user(State(pool): State<Pool<Sqlite>>, Json(user): Json<User>) -> impl IntoResponse {
    let result =
        repo_create_user(pool,
                         User {
                             id: Uuid::new_v4().to_string(),
                             name: user.name,
                             bio: user.bio,
                             pfp: user.pfp,
                             cv: user.cv,
                             is_banned: false  })
            .await;
    (StatusCode::OK, json!({"rows_affected":result.rows_affected()}).to_string()).into_response()
}