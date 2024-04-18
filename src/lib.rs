pub mod database;
pub mod endpoints;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use user::User;


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

