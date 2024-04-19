use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, Json
};
use serde::Deserialize;

use crate::endpoints::{errors::internal_error, AppState};

use super::model::User;

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

pub async fn edit_user(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query!(
        "UPDATE users SET name = ?, email= ? WHERE id = ?",
        params.name,
        params.email,
        id
    )
    .execute(&state.db_pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(User {
        id,
        name: params.name,
        email: params.email,
    }))
}
