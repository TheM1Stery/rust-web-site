pub mod database;
mod user;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use user::User;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

pub async fn index() -> Html<&'static str> {
    Html("<h1>Rust website!<h1>")
}

pub async fn healthcheck() -> Response {
    String::from("OK").into_response()
}

#[derive(Deserialize)]
pub struct UserParams {
    name: String,
    email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email",
        params.name,
        params.email
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(user))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?1", id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(internal_error)?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db_pool)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
