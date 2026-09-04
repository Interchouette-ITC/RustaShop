//! Cart and checkout repository edge coverage for `SeaORM`.

use rustashop_domain::{CartLine, CartStatus, Currency, Money};
use rustashop_persist_seaorm::{migrate, seed_catalog, SeaOrmCatalogRepository};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use serenade_contracts::{CartRepository, PersistenceError};

const HOODIE_VARIANT: &str = "33333333-3333-3333-3333-333333333331";
const MUG_VARIANT: &str = "33333333-3333-3333-3333-333333333332";
const SCHEMA_LOCK: i64 = 874_521;

async fn exclusive_repo() -> Option<(DatabaseConnection, SeaOrmCatalogRepository)> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL is not set");
        return None;
    };
    let mut options = ConnectOptions::new(url);
    options.max_connections(1);
    let db = Database::connect(options).await.expect("connect");
    db.execute_unprepared(&format!("SELECT pg_advisory_lock({SCHEMA_LOCK})"))
        .await
        .expect("lock");
    db.execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .await
        .expect("reset");
    migrate(&db).await.expect("migrate");
    seed_catalog(&db).await.expect("seed");
    Some((db.clone(), SeaOrmCatalogRepository::new(db)))
}

async fn unlock(db: &DatabaseConnection) {
    db.execute_unprepared(&format!("SELECT pg_advisory_unlock({SCHEMA_LOCK})"))
        .await
        .expect("unlock");
}

async fn cart_with_hoodie(repo: &SeaOrmCatalogRepository) -> rustashop_domain::Cart {
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
    repo.find_cart_by_id(&cart.id)
        .await
        .expect("reload")
        .expect("cart")
}

#[tokio::test]
async fn cart_token_save_delete_and_missing_paths() {
    let Some((db, repo)) = exclusive_repo().await else {
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
    assert!(matches!(
        repo.find_cart_by_id("not-a-uuid").await,
        Err(PersistenceError::InvalidInput { .. })
    ));
    assert!(repo
        .find_variant_for_cart("00000000-0000-0000-0000-000000000000")
        .await
        .expect("variant")
        .is_none());

    let with_line = cart_with_hoodie(&repo).await;
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
    unlock(&db).await;
}

#[tokio::test]
async fn checkout_and_order_edge_paths() {
    let Some((db, repo)) = exclusive_repo().await else {
        return;
    };
    let currency = Currency::new("EUR").expect("EUR");
    let cart = cart_with_hoodie(&repo).await;

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
        .checkout_cart(&cart.id, Some("seaorm-cart-key-1"))
        .await
        .expect("checkout");
    assert_eq!(order.lines.len(), 1);
    let replay = repo
        .checkout_cart(&cart.id, Some("seaorm-cart-key-1"))
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
    unlock(&db).await;
}

#[tokio::test]
async fn order_state_update_and_invalid_ids() {
    let Some((db, repo)) = exclusive_repo().await else {
        return;
    };
    let cart = cart_with_hoodie(&repo).await;
    let order = repo
        .checkout_cart(&cart.id, Some("seaorm-state-key"))
        .await
        .expect("checkout");

    let paid = repo
        .update_order_state(&order.id, rustashop_domain::OrderState::Paid)
        .await
        .expect("paid");
    assert_eq!(paid.state, "paid");
    assert!(matches!(
        repo.update_order_state("not-a-uuid", rustashop_domain::OrderState::Paid)
            .await,
        Err(PersistenceError::InvalidInput { .. })
    ));
    assert!(matches!(
        repo.update_order_state(
            "00000000-0000-0000-0000-000000000000",
            rustashop_domain::OrderState::Paid
        )
        .await,
        Err(PersistenceError::NotFound {
            entity: "order",
            ..
        })
    ));
    assert!(matches!(
        repo.get_order("not-a-uuid").await,
        Err(PersistenceError::InvalidInput { .. })
    ));
    assert!(matches!(
        repo.checkout_cart("not-a-uuid", None).await,
        Err(PersistenceError::InvalidInput { .. })
    ));

    let unknown_key = repo
        .checkout_cart(&cart.id, Some("seaorm-unknown-after-checkout"))
        .await
        .expect_err("unknown key after checkout");
    assert!(matches!(
        unknown_key,
        PersistenceError::Conflict {
            constraint: "cart_status"
        }
    ));
    unlock(&db).await;
}

#[tokio::test]
async fn checkout_multi_line_and_list_offset() {
    let Some((db, repo)) = exclusive_repo().await else {
        return;
    };
    let currency = Currency::new("EUR").expect("EUR");
    let _first = {
        let cart = cart_with_hoodie(&repo).await;
        repo.checkout_cart(&cart.id, Some("seaorm-offset-1"))
            .await
            .expect("first order")
    };

    let multi = repo.create_cart(&currency).await.expect("multi cart");
    let (hoodie, hoodie_name) = repo
        .find_variant_for_cart(HOODIE_VARIANT)
        .await
        .expect("hoodie")
        .expect("hoodie variant");
    let (mug, mug_name) = repo
        .find_variant_for_cart(MUG_VARIANT)
        .await
        .expect("mug")
        .expect("mug variant");
    let mut multi_loaded = repo
        .find_cart_by_id(&multi.id)
        .await
        .expect("load multi")
        .expect("cart");
    multi_loaded.lines.push(CartLine {
        id: String::new(),
        cart_id: multi.id.clone(),
        variant_id: hoodie.id,
        quantity: 1,
        unit_price: hoodie.price,
        product_name: hoodie_name,
        variant_sku: hoodie.sku,
    });
    multi_loaded.lines.push(CartLine {
        id: String::new(),
        cart_id: multi.id.clone(),
        variant_id: mug.id,
        quantity: 2,
        unit_price: mug.price,
        product_name: mug_name,
        variant_sku: mug.sku,
    });
    repo.save_cart(&multi_loaded).await.expect("save multi");
    let multi_order = repo
        .checkout_cart(&multi.id, None)
        .await
        .expect("checkout multi");
    assert_eq!(multi_order.lines.len(), 2);

    let offset = repo
        .list_orders(serenade_contracts::PageRequest {
            limit: 1,
            offset: 1,
        })
        .await
        .expect("offset");
    assert_eq!(offset.len(), 1);
    unlock(&db).await;
}

#[tokio::test]
async fn cart_save_preserves_line_id_and_rejects_bad_ids() {
    let Some((db, repo)) = exclusive_repo().await else {
        return;
    };
    let with_line = cart_with_hoodie(&repo).await;
    let line_id = with_line.lines[0].id.clone();
    assert_ne!(line_id, "");
    CartRepository::save(&repo, &with_line)
        .await
        .expect("re-save");
    let again = repo
        .find_cart_by_id(&with_line.id)
        .await
        .expect("reload 2")
        .expect("cart");
    assert_eq!(again.lines[0].id, line_id);

    let bad = "not-a-uuid".to_owned();
    assert!(matches!(
        CartRepository::delete(&repo, &bad).await,
        Err(PersistenceError::InvalidInput { .. })
    ));
    assert!(matches!(
        repo.find_variant_for_cart("not-a-uuid").await,
        Err(PersistenceError::InvalidInput { .. })
    ));
    unlock(&db).await;
}
