mod config;
mod handlers;
use handlers::*;
use std::{ops::Deref, sync::Arc};

use axum::{
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;

#[derive(Clone)]
pub(crate) struct AppState {
    inner: Arc<AppConfigInner>,
}
#[allow(unused)]
pub(crate) struct AppConfigInner {
    pub(crate) config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat:id",
            patch(update_chat_handler).delete(delete_chat_handler),
        )
        .route(
            "/message",
            get(list_message_handler).post(create_message_handler),
        )
        .route(
            "/message:id",
            patch(update_message_handler).delete(delete_message_handler),
        );
    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}

impl Deref for AppState {
    type Target = AppConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppConfigInner { config }),
        }
    }
}
