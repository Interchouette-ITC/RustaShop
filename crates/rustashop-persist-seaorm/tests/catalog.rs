use rustashop_persist_seaorm::{migrate, SeaOrmCatalogRepository};
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use serenade_contracts::{CategoryRepository, PageRequest, ProductRepository};

#[tokio::test]
async fn seaorm_catalog_lists_and_finds_seeded_rows() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return;
    };
    let db = Database::connect(&url).await.expect("connect");
    migrate(&db).await.expect("migrate");
    db.execute_unprepared("SELECT pg_advisory_lock(874512)")
        .await
        .expect("lock");
    db.execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .await
        .expect("reset schema");
    migrate(&db).await.expect("migrate after reset");
    db.execute_unprepared(
        "INSERT INTO category (id, slug, name) VALUES ('11111111-1111-1111-1111-111111111111', 'apparel', 'Apparel')",
    )
    .await
    .expect("seed category");
    db.execute_unprepared(
        "INSERT INTO product (id, category_id, slug, name, enabled) VALUES
         ('22222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 'hoodie', 'Hoodie', TRUE)",
    )
    .await
    .expect("seed product");

    let repo = SeaOrmCatalogRepository::new(db);
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
}
