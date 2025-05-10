use crate::models::user::User;
use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres};

pub async fn read_users(pool: Pool<Postgres>) -> Vec<User> {
    let users: Vec<User> = sqlx::query_as("select * from users")
        .fetch_all(&pool)
        .await
        .unwrap();

    users
}

pub async fn read_user_by_id(pool: Pool<Postgres>, id: String) -> User {
    let user = sqlx::query_as("select * from users where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    user
}

pub async fn create_user(pool: Pool<Postgres>, user: User) -> PgQueryResult {
    let result = sqlx::query(
        "insert into users (id, name, bio, pfp, cv, is_banned) values ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.bio)
    .bind(&user.pfp)
    .bind(&user.cv)
    .bind(&user.is_banned)
    .execute(&pool)
    .await
    .unwrap();
    result
}

pub async fn update_user(pool: Pool<Postgres>, user: User) -> PgQueryResult {
    let result =
        sqlx::query("update users set name=$1, bio=$2, pfp=$3, cv=$4, is_banned=$5 where id=$6")
            .bind(&user.name)
            .bind(&user.bio)
            .bind(&user.pfp)
            .bind(&user.cv)
            .bind(&user.is_banned)
            .bind(&user.id)
            .execute(&pool)
            .await
            .unwrap();
    result
}

pub async fn delete_user(pool: Pool<Postgres>, id: String) -> PgQueryResult {
    let result = sqlx::query("delete from users where id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    result
}
