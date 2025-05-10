mod handlers;
mod models;
mod repos;
mod routes;
mod config;
mod doc;

use crate::routes::user::user_routes;
use axum::routing::get;
use axum::{serve, Router};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = config::db().await;
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let app = app(pool).await;
    println!("Listening on http://{}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}

async fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { json!({"hello": "world"}).to_string() }))
        .merge(user_routes())
        .with_state(pool)
}
