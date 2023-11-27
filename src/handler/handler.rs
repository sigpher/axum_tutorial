use axum::Json;
use reqwest::StatusCode;

use crate::model::model::{User, CreateUser};

pub async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 0,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}
