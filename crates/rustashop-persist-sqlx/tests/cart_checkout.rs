//! Cart and checkout repository edge coverage for `SQLx`.

use rustashop_domain::{CartLine, CartStatus, Currency, Money};
use rustashop_persist_sqlx::{migrate, seed_catalog, SqlxCatalogRepository};
use serenade_contracts::{CartRepository, PersistenceError};
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

const HOODIE_VARIANT: &str = "33333333-3333-3333-3333-333333333331";
const SCHEMA_LOCK: i64 = 874_520;

async fn exclusive_repo() -> Option<(PgPool, SqlxCatalogRepository)> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return None;
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
        .expect("drop");
    sqlx::query("CREATE SCHEMA public")
        .execute(&pool)
        .await
        .expect("create");
    migrate(&pool).await.expect("migrate");
    seed_catalog(&pool).await.expect("seed");
    Some((pool.clone(), SqlxCatalogRepository::new(pool)))
}

async fn unlock(pool: &PgPool) {
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(SCHEMA_LOCK)
        .execute(pool)
        .await
        .expect("unlock");
}

#[tokio::test]
async fn cart_token_save_delete_and_missing_paths() {
    let Some((pool, repo)) = exclusive_repo().await else {
        return;
    };
    let currency = Currency::new("EUR").expect("EUR");
    let cart = repo.create_cart(&currency).await.expect("create");
    assert!(CartRepository::find_by_token(&repo, &cart.token)
        .await
        .expect("token")
        .is_some());
    assert!(CartRepository::find_by_token(&repo, "missing-token")
        .await
        .expect("missing")
        .is_none());
    assert!(repo
        .find_cart_by_id("00000000-0000-0000-0000-000000000000")
        .await
        .expect("id")
        .is_none());
    assert!(repo
        .find_variant_for_cart("00000000-0000-0000-0000-000000000000")
        .await
        .expect("variant")
        .is_none());

    let (variant, product_name) = repo
        .find_variant_for_cart(HOODIE_VARIANT)
        .await
        .expect("find variant")
        .expect("hoodie");
    let mut loaded = repo
        .find_cart_by_id(&cart.id)
        .await
        .expect("load")
        .expect("cart");
    loaded.lines.push(CartLine {
        id: String::new(),
        cart_id: loaded.id.clone(),
        variant_id: variant.id.clone(),
        quantity: 1,
        unit_price: variant.price.clone(),
        product_name,
        variant_sku: variant.sku.clone(),
    });
    CartRepository::save(&repo, &loaded).await.expect("save");
    let with_line = repo
        .find_cart_by_id(&cart.id)
        .await
        .expect("reload")
        .expect("cart");
    assert_eq!(with_line.lines.len(), 1);
    assert_ne!(with_line.lines[0].id, "");

    let empty = repo.create_cart(&currency).await.expect("empty cart");
    CartRepository::delete(&repo, &empty.id)
        .await
        .expect("delete");
    assert!(repo
        .find_cart_by_id(&empty.id)
        .await
        .expect("after delete")
        .is_none());

    let mut ghost = with_line;
    ghost.id = "00000000-0000-0000-0000-000000000099".to_owned();
    ghost.status = CartStatus::Open;
    ghost.lines.clear();
    let save_missing = repo.save_cart(&ghost).await.expect_err("missing save");
    assert!(matches!(
        save_missing,
        PersistenceError::NotFound { entity: "cart", .. }
    ));
    let _ = Money::new(0, currency);
    unlock(&pool).await;
}

#[tokio::test]
async fn checkout_and_order_edge_paths() {
    let Some((pool, repo)) = exclusive_repo().await else {
        return;
    };
    let currency = Currency::new("EUR").expect("EUR");
    let cart = repo.create_cart(&currency).await.expect("create");
    let (variant, product_name) = repo
        .find_variant_for_cart(HOODIE_VARIANT)
        .await
        .expect("find variant")
        .expect("hoodie");
    let mut loaded = repo
        .find_cart_by_id(&cart.id)
        .await
        .expect("load")
        .expect("cart");
    loaded.lines.push(CartLine {
        id: String::new(),
        cart_id: loaded.id.clone(),
        variant_id: variant.id,
        quantity: 1,
        unit_price: variant.price,
        product_name,
        variant_sku: variant.sku,
    });
    repo.save_cart(&loaded).await.expect("save");

    let missing = repo
        .checkout_cart("00000000-0000-0000-0000-000000000000", None)
        .await
        .expect_err("missing cart");
    assert!(matches!(
        missing,
        PersistenceError::NotFound { entity: "cart", .. }
    ));

    let empty = repo.create_cart(&currency).await.expect("empty cart");
    let empty_err = repo
        .checkout_cart(&empty.id, None)
        .await
        .expect_err("empty");
    assert!(matches!(empty_err, PersistenceError::InvalidInput { .. }));

    let order = repo
        .checkout_cart(&cart.id, Some("sqlx-cart-key-1"))
        .await
        .expect("checkout");
    assert_eq!(order.lines.len(), 1);
    let replay = repo
        .checkout_cart(&cart.id, Some("sqlx-cart-key-1"))
        .await
        .expect("replay");
    assert_eq!(replay.id, order.id);

    let conflict = repo
        .checkout_cart(&cart.id, None)
        .await
        .expect_err("checked out");
    assert!(matches!(
        conflict,
        PersistenceError::Conflict {
            constraint: "cart_status"
        }
    ));

    let get = repo.get_order(&order.id).await.expect("get order");
    assert_eq!(get.id, order.id);
    let missing_order = repo
        .get_order("00000000-0000-0000-0000-000000000000")
        .await
        .expect_err("missing order");
    assert!(matches!(
        missing_order,
        PersistenceError::NotFound {
            entity: "order",
            ..
        }
    ));

    let listed = repo
        .list_orders(serenade_contracts::PageRequest::first(10))
        .await
        .expect("list");
    assert_ne!(listed.len(), 0);
    unlock(&pool).await;
}
