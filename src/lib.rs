use axum::{response::{Html, IntoResponse, Response}, Json};
use serde::Serialize;

pub async fn index() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

#[derive(Serialize)]
pub struct Test {
    text: String,
    salam: i32
}

pub async fn return_json() -> Json<Test> {
    Json(Test {
        text: String::from("salam"),
        salam: 1337
    })
}


pub async fn healthcheck() -> Response {
    String::from("OK").into_response()
}

