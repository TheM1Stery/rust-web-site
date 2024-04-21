use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::endpoints::{errors::internal_error, AppState};

use super::model::User;

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
