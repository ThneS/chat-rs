mod auth;
mod chat;
mod messages;
use axum::response::IntoResponse;

pub(crate) use auth::*;

pub(crate) async fn index_handler() -> impl IntoResponse {
    "index"
}
