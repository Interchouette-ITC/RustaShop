use rustashop_persist_seaorm::migrate_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    migrate_from_env().await?;
    Ok(())
}
