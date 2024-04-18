use axum::Router;

mod get;
mod model;

pub fn user_router() -> Router{
    Router::new()
}
