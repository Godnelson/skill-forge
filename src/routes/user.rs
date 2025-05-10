use axum::{Router};
use axum::routing::{post};
use sqlx::{Pool, Sqlite};
use crate::handlers::user::create_user;

pub fn user_routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/user/create", post(create_user))
}