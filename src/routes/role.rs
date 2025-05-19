use axum::Router;
use axum::routing::{delete, get, post, put};
use sqlx::{Pool, Postgres};
use crate::handlers::role::{create_role, delete_role, read_role_by_id, read_roles, update_role};

pub fn role_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/role", get(read_roles))
        .route("/role/{id}", get(read_role_by_id))
        .route("/role/create", post(create_role))
        .route("/role/update", put(update_role))
        .route("/role/delete", delete(delete_role))
}