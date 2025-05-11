use crate::models::user::{User, UserDTO};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Pool, Postgres};

pub async fn read_users(pool: Pool<Postgres>) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as("select * from users")
        .fetch_all(&pool)
        .await;

    Ok(users?)
}

pub async fn read_user_by_id(pool: Pool<Postgres>, id: String) -> Result<Option<User>, Error> {
    let user = sqlx::query_as("select * from users where id=$1")
        .bind(id)
        .fetch_optional(&pool)
        .await;
    user
}

pub async fn create_user(pool: Pool<Postgres>, user: User) -> Result<PgQueryResult, Error> {
    let result = sqlx::query(
        "insert into users (id, name, email, password, bio, pfp, cv, is_banned) values ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.bio)
        .bind(&user.pfp)
        .bind(&user.cv)
        .bind(&user.is_banned)
        .execute(&pool)
    .await;
    
    Ok(result?)
}

pub async fn update_user(pool: Pool<Postgres>, user: UserDTO) -> Result<PgQueryResult, Error> {
    let result =
        sqlx::query("update users set name=$1, email=$2, bio=$3, pfp=$4, cv=$5, is_banned=$6 where id=$7")
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.bio)
            .bind(&user.pfp)
            .bind(&user.cv)
            .bind(&user.is_banned)
            .bind(&user.id)
            .execute(&pool)
            .await?;
    Ok(result)
}

pub async fn delete_user(pool: Pool<Postgres>, id: String) -> Result<PgQueryResult, Error> {
    let result = sqlx::query("delete from users where id=$1")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
}
