use crate::models::user::{User, UserDTO, UserToCreate};
use crate::repos::user::create_user as repo_create_user;
use crate::repos::user::delete_user as repo_delete_user;
use crate::repos::user::read_user_by_id as repo_read_users_by_id;
use crate::repos::user::read_users as repo_read_users;
use crate::repos::user::update_user as repo_update_user;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get users success", body = Vec<UserDTO>)
    )
)]
pub async fn read_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let users = repo_read_users(pool).await;
    match users {
        Ok(users) => {
            let mut users_to_return: Vec<UserDTO> = Vec::new();
            for user in users {
                users_to_return.push(UserDTO {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    bio: user.bio,
                    pfp: user.pfp,
                    cv: user.cv,
                    is_banned: user.is_banned,
                })
            }
            (StatusCode::OK, serde_json::to_string_pretty(&users_to_return).unwrap())
        }
            .into_response(),
        Err(err) => (StatusCode::OK, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    responses(
        (status = 200, description = "Get users success", body = UserDTO),
        (status = 404, description = "Not found", body = String)
    ),
    params(
            ("id" = String, Path, description = "User database id to get User for"),
    )
)]
pub async fn read_users_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = repo_read_users_by_id(pool, id)
        .await
        .unwrap()
        .ok_or("User not found");

    match user {
        Ok(user) => {
            let user_to_return = UserDTO {
                id: user.id,
                name: user.name,
                email: user.email,
                bio: user.bio,
                pfp: user.pfp,
                cv: user.cv,
                is_banned: user.is_banned,
            };
            (StatusCode::OK, serde_json::to_string_pretty(&user_to_return).unwrap()).into_response()
        },
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/user/create",
    responses(
        (status = 201, description = "Create user success", body = UserDTO),
        (status = 422, description = "Unprocessable Entity", body = String),
        (status = 400, description = "Bad request", body = String)
    )
)]
pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<UserToCreate>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: user.name,
        email: user.email,
        password: password_hash,
        bio: user.bio,
        pfp: user.pfp,
        cv: user.cv,
        is_banned: false,
    };
    let result = repo_create_user(pool, user.clone()).await;

    let user_to_return = UserDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        bio: user.bio,
        pfp: user.pfp,
        cv: user.cv,
        is_banned: user.is_banned,
    };
    match result {
        Ok(_) => (
            StatusCode::CREATED,
            serde_json::to_string_pretty(&user_to_return).unwrap(),
        )
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/user/update",
    responses(
        (status = 200, description = "Update user success", body = UserDTO),
        (status = 400, description = "Bad request", body = String),
    (status = 404, description = "User not found", body = String)
    )
)]
pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<UserDTO>,
) -> impl IntoResponse {
    let result = repo_update_user(pool, user.clone()).await;
    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "User not found").into_response()
            }else {
                (StatusCode::OK, serde_json::to_string_pretty(&user).unwrap()).into_response()
            }
        },
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/user/delete/{id}",
    responses(
        (status = 204, description = "Delete user success", body = String),
        (status = 400, description = "Bad request", body = String),
        (status = 404, description = "User not found", body = String)
    ),
    params(
            ("id" = String, Path, description = "User database id to delete User for"),
    )
)]
pub async fn delete_user(State(pool): State<Pool<Postgres>>, Path(id): Path<String>) -> impl IntoResponse {
    let result = repo_delete_user(pool, id).await;
    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "User not found").into_response()
            } else {
                StatusCode::NO_CONTENT.into_response()
            }
        }
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
