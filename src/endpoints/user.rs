use axum::Router;

use self::{
    get::{get_all_users, get_user},
    modify::{create_user, edit_user},
};
use axum::routing::get;

use super::AppState;

mod get;
mod model;
mod modify;

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user).put(edit_user))
        .route("/", get(get_all_users).post(create_user))
}
