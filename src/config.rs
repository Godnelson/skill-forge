use sqlx::postgres::PgPoolOptions;
use sqlx::sqlx_macros::migrate;
use sqlx::{Pool, Postgres};

pub async fn db() -> Pool<Postgres> {
    let database_url = format!(
        "postgres://{}:{}@localhost/{}",
        std::env::var("DATABASE_USER").unwrap(),
        std::env::var("DATABASE_PASSWORD").unwrap(),
        std::env::var("DATABASE_NAME").unwrap()
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();

    migrate!("./migrations");

    pool
}
