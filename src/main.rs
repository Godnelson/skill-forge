mod models;
mod repos;
mod handlers;
mod routes;

use std::path::Path;
use models::user::User;
use axum::{serve, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::net::TcpListener;
use crate::routes::user::user_routes;

#[tokio::main]
async fn main() {
    let pool = db().await;
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let app = app(pool).await;
    println!("Listening on http://{}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}


async fn app(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/", get(|| async { json!({"hello": "world"}).to_string()}))
        .route("/cu", get(|| async { json!({"cu": "cu"}).to_string() }))
        .route("/test_db", get(test_db))
        .merge(user_routes())
        .with_state(pool)
}

async fn test_db(State(pool): State<Pool<Sqlite>>, Json(cu): Json<User>) -> impl IntoResponse {
    let users:Vec<User> = sqlx::query_as("select * from users").fetch_all(&pool).await.unwrap();
    println!("{:?}", cu);
    let json_data: String = serde_json::to_string_pretty(&users).unwrap();
    (StatusCode::OK, json_data).into_response()
}

async fn db() -> Pool<Sqlite> {
    let options = SqliteConnectOptions::new()
        .filename("database/skillforge.db")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await.unwrap();
    sqlx::migrate::Migrator::new(Path::new("migrations"))
        .await.unwrap()
        .run(&pool)
        .await.unwrap();

    pool
}
