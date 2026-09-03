//! Integration tests for the `SQLx` catalog repository.

use rustashop_persist_sqlx::{migrate, SqlxCatalogRepository};
use serenade_contracts::{CategoryRepository, PageRequest, ProductRepository};
use sqlx::postgres::PgPoolOptions;

const SCHEMA_LOCK: i64 = 874_512;

#[tokio::test]
async fn sqlx_catalog_lists_and_finds_seeded_rows() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .expect("connect");
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(SCHEMA_LOCK)
        .execute(&pool)
        .await
        .expect("lock");
    sqlx::query("DROP SCHEMA public CASCADE")
        .execute(&pool)
        .await
        .expect("drop schema");
    sqlx::query("CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("create schema");
    migrate(&pool).await.expect("migrate");
    sqlx::query(
        "INSERT INTO category (id, slug, name) VALUES ('11111111-1111-1111-1111-111111111111', 'apparel', 'Apparel')",
    )
    .execute(&pool)
    .await
    .expect("seed category");
    sqlx::query(
        "INSERT INTO product (id, category_id, slug, name, enabled) VALUES
         ('22222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 'hoodie', 'Hoodie', TRUE)",
    )
    .execute(&pool)
    .await
    .expect("seed product");

    let repo = SqlxCatalogRepository::new(pool.clone());
    let product = ProductRepository::find_by_slug(&repo, "hoodie")
        .await
        .expect("slug")
        .expect("hoodie");
    assert_eq!(product.name, "Hoodie");
    let by_id = ProductRepository::find_by_id(&repo, &product.id)
        .await
        .expect("id")
        .expect("row");
    assert_eq!(by_id.slug, "hoodie");
    let listed = ProductRepository::list(&repo, PageRequest::first(10))
        .await
        .expect("list");
    assert_eq!(listed.len(), 1);

    let category = CategoryRepository::find_by_slug(&repo, "apparel", None)
        .await
        .expect("category slug")
        .expect("apparel");
    assert_eq!(category.name, "Apparel");
    let children = CategoryRepository::list_children(&repo, None, PageRequest::first(10))
        .await
        .expect("children");
    assert_eq!(children.len(), 1);

    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(SCHEMA_LOCK)
        .execute(&pool)
        .await
        .expect("unlock");
}
