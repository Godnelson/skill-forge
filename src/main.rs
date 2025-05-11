mod config;
mod handlers;
mod models;
mod repos;
mod routes;

use crate::routes::user::user_routes;
use axum::routing::get;
use axum::{serve, Router};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::user::read_users,
        handlers::user::read_users_by_id,
        handlers::user::create_user,
        handlers::user::update_user,
        handlers::user::delete_user,
    ),
    components(
        schemas(models::user::User, models::user::UserToCreate),
    ),
    tags(
        (name = "User", description = "Operações com usuários"),
    )
)]
struct ApiDoc;

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
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
