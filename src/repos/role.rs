use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use crate::models::role::Role;
pub async fn read_roles(pool: Pool<Postgres>) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = sqlx::query_as("SELECT * FROM role")
        .fetch_all(&pool)
        .await?;
    Ok(roles)
}

pub async fn read_role_by_id(pool: Pool<Postgres>, id: String) -> Result<Option<Role>, Error> {
    let role = sqlx::query_as("SELECT * FROM role WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    Ok(role)
}

pub async fn create_role(pool: Pool<Postgres>, role: &Role) -> Result<PgQueryResult, Error> {
    let result = sqlx::query("INSERT INTO role (id, name) values ($1, $2)")
        .bind(&role.id)
        .bind(&role.name)
        .execute(&pool)
        .await?;
    Ok(result)
}

pub async fn update_role(pool: Pool<Postgres>, role: &Role) -> Result<PgQueryResult, Error> {
    let result = sqlx::query("UPDATE role SET name = $1 WHERE id = $2")
        .bind(&role.name)
        .bind(&role.id)
        .execute(&pool)
        .await?;
    Ok(result)
}

pub async fn delete_role(pool: Pool<Postgres>, id: String, role: &Role) -> Result<PgQueryResult, Error> {
    let result = sqlx::query("DELETE FROM role  WHERE id = $1")
        .bind(&id)
        .execute(&pool)
        .await?;
    Ok(result)
}