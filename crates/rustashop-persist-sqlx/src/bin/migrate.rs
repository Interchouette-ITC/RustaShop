//! Runs embedded `SQLx` migrations using `DATABASE_URL`.

use rustashop_persist_sqlx::migrate_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    migrate_from_env().await?;
    println!("Migrations applied (SQLx).");
    Ok(())
}
