//! Schema-break fault injection for `PersistenceError::Internal` arms (`SeaORM`).

use rustashop_domain::Currency;
use rustashop_persist_seaorm::{migrate, seed_catalog, SeaOrmCatalogRepository};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use serenade_contracts::{CategoryRepository, PageRequest, PersistenceError, ProductRepository};

/// Same lock key as the `SQLx` fault suite so both adapters serialize on one Postgres.
const SCHEMA_LOCK: i64 = 874_530;
const HOODIE_VARIANT: &str = "33333333-3333-3333-3333-333333333331";

async fn exclusive_repo() -> Option<(DatabaseConnection, SeaOrmCatalogRepository)> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return None;
    };
    let mut options = ConnectOptions::new(url);
    options.max_connections(2);
    let db = Database::connect(options).await.expect("connect");
    db.execute_unprepared(&format!("SELECT pg_advisory_lock({SCHEMA_LOCK})"))
        .await
        .expect("lock");
    reset_schema(&db).await;
    Some((db.clone(), SeaOrmCatalogRepository::new(db)))
}

async fn reset_schema(db: &DatabaseConnection) {
    db.execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .await
        .expect("reset");
    migrate(db).await.expect("migrate");
    seed_catalog(db).await.expect("seed");
}

async fn unlock(db: &DatabaseConnection) {
    db.execute_unprepared(&format!("SELECT pg_advisory_unlock({SCHEMA_LOCK})"))
        .await
        .expect("unlock");
}

fn assert_internal(err: &PersistenceError) {
    assert!(
        matches!(err, PersistenceError::Internal { .. }),
        "expected Internal, got {err:?}"
    );
}

async fn seed_cart_with_hoodie(repo: &SeaOrmCatalogRepository) -> rustashop_domain::Cart {
    let currency = Currency::new("EUR").expect("EUR");
    let cart = repo.create_cart(&currency).await.expect("create");
    let (variant, product_name) = repo
        .find_variant_for_cart(HOODIE_VARIANT)
        .await
        .expect("variant")
        .expect("hoodie");
    let mut loaded = repo
        .find_cart_by_id(&cart.id)
        .await
        .expect("load")
        .expect("cart");
    loaded.lines.push(rustashop_domain::CartLine {
        id: String::new(),
        cart_id: loaded.id.clone(),
        variant_id: variant.id,
        quantity: 1,
        unit_price: variant.price,
        product_name,
        variant_sku: variant.sku,
    });
    repo.save_cart(&loaded).await.expect("save");
    loaded
}

async fn drop_commerce_tables(db: &DatabaseConnection) {
    db.execute_unprepared(
        r#"
DROP TABLE IF EXISTS order_line CASCADE;
DROP TABLE IF EXISTS "order" CASCADE;
DROP TABLE IF EXISTS cart_line CASCADE;
DROP TABLE IF EXISTS cart CASCADE;
DROP TABLE IF EXISTS product_variant CASCADE;
DROP TABLE IF EXISTS product CASCADE;
DROP TABLE IF EXISTS category CASCADE;
"#,
    )
    .await
    .expect("drop tables");
}

async fn assert_reads_are_internal(repo: &SeaOrmCatalogRepository, cart: &rustashop_domain::Cart) {
    let currency = Currency::new("EUR").expect("EUR");
    assert_internal(&repo.find_cart_by_id(&cart.id).await.expect_err("find cart"));
    assert_internal(
        &serenade_contracts::CartRepository::find_by_token(repo, &cart.token)
            .await
            .expect_err("token"),
    );
    assert_internal(&repo.create_cart(&currency).await.expect_err("create"));
    assert_internal(
        &repo
            .checkout_cart(&cart.id, Some("seaorm-internal-key"))
            .await
            .expect_err("checkout"),
    );
    assert_internal(
        &repo
            .find_variant_for_cart(HOODIE_VARIANT)
            .await
            .expect_err("variant"),
    );
    assert_internal(
        &ProductRepository::list(repo, PageRequest::first(10))
            .await
            .expect_err("products"),
    );
    assert_internal(
        &CategoryRepository::list_children(repo, None, PageRequest::first(10))
            .await
            .expect_err("categories"),
    );
    assert_internal(
        &repo
            .get_order("00000000-0000-0000-0000-000000000001")
            .await
            .expect_err("order"),
    );
    assert_internal(
        &repo
            .list_all_products(PageRequest::first(5))
            .await
            .expect_err("admin products"),
    );
    assert_internal(
        &repo
            .list_variants_for_product("22222222-2222-2222-2222-222222222221")
            .await
            .expect_err("variants"),
    );
}

#[tokio::test]
async fn dropped_tables_surface_internal_on_reads_and_checkout() {
    let Some((db, repo)) = exclusive_repo().await else {
        return;
    };
    let cart = seed_cart_with_hoodie(&repo).await;
    drop_commerce_tables(&db).await;
    assert_reads_are_internal(&repo, &cart).await;
    reset_schema(&db).await;
    unlock(&db).await;
}
