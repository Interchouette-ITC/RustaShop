//! Facade coverage for `catalog_from_env`.

use rustashop_persist::{catalog_from_env, PersistenceFactory};

#[tokio::test]
async fn factory_and_free_fns_open_catalog() {
    let Ok(_) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return;
    };
    let _ = PersistenceFactory
        .catalog_from_env()
        .await
        .expect("factory catalog");
    let _ = catalog_from_env().await.expect("catalog");
}
