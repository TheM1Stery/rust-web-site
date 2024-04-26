use axum::response::{Html, IntoResponse, Response};

pub async fn index() -> Html<&'static str> {
    Html("<h1>Rust website!<h1>")
}

pub async fn test() -> Html<&'static str> {
    Html("<div>Rust website 2!</div>")
}

pub async fn healthcheck() -> Response {
    String::from("OK").into_response()
}
