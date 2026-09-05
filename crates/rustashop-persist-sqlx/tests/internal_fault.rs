//! Schema-break fault injection for `PersistenceError::Internal` arms (`SQLx`).

use rustashop_domain::Currency;
use rustashop_persist_sqlx::{migrate, seed_catalog, SqlxCatalogRepository};
use serenade_contracts::{CategoryRepository, PageRequest, PersistenceError, ProductRepository};
use sqlx::postgres::{PgPool, PgPoolOptions};

const SCHEMA_LOCK: i64 = 874_530;
const HOODIE_VARIANT: &str = "33333333-3333-3333-3333-333333333331";

async fn exclusive_repo() -> Option<(PgPool, SqlxCatalogRepository)> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return None;
    };
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("connect");
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(SCHEMA_LOCK)
        .execute(&pool)
        .await
        .expect("lock");
    reset_schema(&pool).await;
    Some((pool.clone(), SqlxCatalogRepository::new(pool)))
}

async fn reset_schema(pool: &PgPool) {
    sqlx::query("DROP SCHEMA public CASCADE")
        .execute(pool)
        .await
        .expect("drop");
    sqlx::query("CREATE SCHEMA public")
        .execute(pool)
        .await
        .expect("create");
    migrate(pool).await.expect("migrate");
    seed_catalog(pool).await.expect("seed");
}

async fn unlock(pool: &PgPool) {
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(SCHEMA_LOCK)
        .execute(pool)
        .await
        .expect("unlock");
}

fn assert_internal(err: &PersistenceError) {
    assert!(
        matches!(err, PersistenceError::Internal { .. }),
        "expected Internal, got {err:?}"
    );
}

async fn seed_cart_with_hoodie(repo: &SqlxCatalogRepository) -> rustashop_domain::Cart {
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

async fn drop_commerce_tables(pool: &PgPool) {
    for sql in [
        "DROP TABLE cart_line CASCADE",
        "DROP TABLE cart CASCADE",
        r#"DROP TABLE "order" CASCADE"#,
        "DROP TABLE product_variant CASCADE",
        "DROP TABLE product CASCADE",
        "DROP TABLE category CASCADE",
    ] {
        sqlx::query(sql).execute(pool).await.expect("drop");
    }
}

async fn assert_reads_are_internal(repo: &SqlxCatalogRepository, cart: &rustashop_domain::Cart) {
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
            .checkout_cart(&cart.id, Some("sqlx-internal-key"))
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
    let Some((pool, repo)) = exclusive_repo().await else {
        return;
    };
    let cart = seed_cart_with_hoodie(&repo).await;
    drop_commerce_tables(&pool).await;
    assert_reads_are_internal(&repo, &cart).await;
    reset_schema(&pool).await;
    unlock(&pool).await;
}

#[tokio::test]
async fn closed_pool_surfaces_internal_on_begin() {
    let Some((pool, repo)) = exclusive_repo().await else {
        return;
    };
    let currency = Currency::new("EUR").expect("EUR");
    let cart = repo.create_cart(&currency).await.expect("create");
    unlock(&pool).await;
    pool.close().await;
    assert_internal(
        &repo
            .checkout_cart(&cart.id, None)
            .await
            .expect_err("closed pool checkout"),
    );

    let Ok(url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let fresh = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .expect("reconnect");
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(SCHEMA_LOCK)
        .execute(&fresh)
        .await
        .expect("relock");
    reset_schema(&fresh).await;
    unlock(&fresh).await;
}
