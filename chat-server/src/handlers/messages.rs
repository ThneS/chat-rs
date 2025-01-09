use axum::response::IntoResponse;

pub(crate) async fn create_message_handler() -> impl IntoResponse {
    "create_message"
}
pub(crate) async fn update_message_handler() -> impl IntoResponse {
    "update_message"
}
pub(crate) async fn delete_message_handler() -> impl IntoResponse {
    "delete_message"
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list_message"
}
