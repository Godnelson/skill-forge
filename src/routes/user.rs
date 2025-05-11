use crate::handlers::user::{create_user, delete_user, read_users, read_users_by_id, update_user};
use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::{Pool, Postgres};

pub fn user_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/user", get(read_users))
        .route("/user/{id}", get(read_users_by_id))
        .route("/user/create", post(create_user))
        .route("/user/update", put(update_user))
        .route("/user/delete/{id}", delete(delete_user))
}
