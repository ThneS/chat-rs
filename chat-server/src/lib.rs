mod config;
mod error;
mod handlers;
mod models;
mod utils;
use axum::{
    routing::{get, post},
    Router,
};
pub use config::AppConfig;
use handlers::*;
use std::{ops::Deref, sync::Arc};

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
        .route("/signup", post(signup_handler));
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
