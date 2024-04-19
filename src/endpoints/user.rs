use axum::Router;

use self::{
    get::{get_all_users, get_user},
    modify::create_user,
};
use axum::routing::get;

use super::AppState;

mod get;
mod model;
mod modify;

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user))
        .route("/", get(get_all_users).post(create_user))
}
