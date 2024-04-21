use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub(super) id: i64,
    pub name: String,
    pub email: String,
}
