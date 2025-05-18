use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use crate::repos::user::read_user_by_email;

pub async fn login(State(pool): State<Pool<Postgres>> ,email: String, password: String) -> impl IntoResponse{
    let user = read_user_by_email(pool, email).await;
    
}