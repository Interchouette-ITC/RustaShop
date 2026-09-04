//! Actix HTTP server entry point.

use rustashop_api::run;
use tracing::error;
use tracing_subscriber::{fmt, EnvFilter};

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx::query=warn,actix_server=warn"));
    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_writer(std::io::stdout)
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();
    if let Err(error) = run().await {
        error!("{error}");
        std::process::exit(1);
    }
}
