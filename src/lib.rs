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
    Html("<h1>Hello World!</h1>")
}

#[derive(Serialize)]
pub struct Test {
    text: String,
    salam: i32,
}

pub async fn return_json() -> Json<Test> {
    Json(Test {
        text: String::from("salam"),
        salam: 1337,
    })
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
    let res = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ( ?1, ?2)",
        params.name,
        params.email
    )
    .execute(&state.db_pool)
    .await
    .map_err(internal_error);

    match res {
        Ok(sql_result) => {
            let user = User {
                id: sql_result.last_insert_rowid(),
                name: params.name,
                email: params.email,
            };
            Ok(Json(user))
        }
        Err(e) => Err(e)
    }
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<User>, (StatusCode, String)>{
    let user =
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?1", id)
        .fetch_optional(&state.db_pool)
        .await
        .transpose();

    if user.is_none(){
        return Err((StatusCode::NOT_FOUND, String::from("Not found")));
    }

    match user.unwrap().map_err(internal_error) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
