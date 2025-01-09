use anyhow::{Ok, Result};
use notify_server::get_router;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let addr = "0.0.0.0:6687".to_string();
    info!("Starting server on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let app = get_router();
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
