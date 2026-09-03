//! Actix HTTP server entry point.

use rustashop_api::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
