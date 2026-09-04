//! Browser cart session (same `rs.cartId` key as the Angular shop).

use leptos::prelude::*;

use crate::api::{self, Cart};

const CART_ID_KEY: &str = "rs.cartId";

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn read_cart_id() -> Option<String> {
    local_storage()?.get_item(CART_ID_KEY).ok().flatten()
}

fn write_cart_id(id: &str) {
    if let Some(store) = local_storage() {
        let _ = store.set_item(CART_ID_KEY, id);
    }
}

fn clear_cart_id() {
    if let Some(store) = local_storage() {
        let _ = store.remove_item(CART_ID_KEY);
    }
}

#[derive(Clone, Copy)]
pub struct CartCtx {
    pub cart: RwSignal<Option<Cart>>,
    pub busy: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl CartCtx {
    pub fn provide() -> Self {
        let ctx = Self {
            cart: RwSignal::new(None),
            busy: RwSignal::new(false),
            error: RwSignal::new(None),
        };
        provide_context(ctx);
        ctx
    }

    pub fn line_count(&self) -> usize {
        self.cart.get().map_or(0, |cart| {
            cart.lines
                .iter()
                .map(|line| usize::try_from(line.quantity.max(0)).unwrap_or(0))
                .sum()
        })
    }

    pub async fn ensure_cart(&self) -> Result<Cart, String> {
        if let Some(id) = read_cart_id() {
            match api::get_cart(&id).await {
                Ok(cart) if cart.status == "open" => {
                    self.cart.set(Some(cart.clone()));
                    self.error.set(None);
                    return Ok(cart);
                }
                _ => clear_cart_id(),
            }
        }
        let cart = api::create_cart().await?;
        write_cart_id(&cart.id);
        self.cart.set(Some(cart.clone()));
        self.error.set(None);
        Ok(cart)
    }

    pub async fn refresh(&self) -> Result<(), String> {
        let id = self
            .cart
            .get_untracked()
            .map(|cart| cart.id)
            .or_else(read_cart_id);
        let Some(id) = id else {
            self.cart.set(None);
            return Ok(());
        };
        self.busy.set(true);
        let result = api::get_cart(&id).await;
        self.busy.set(false);
        match result {
            Ok(cart) => {
                write_cart_id(&cart.id);
                self.cart.set(Some(cart));
                self.error.set(None);
                Ok(())
            }
            Err(err) => {
                self.error.set(Some(err.clone()));
                Err(err)
            }
        }
    }

    pub async fn add_line(&self, variant_id: &str, quantity: i32) -> Result<Cart, String> {
        self.busy.set(true);
        self.error.set(None);
        let result: Result<Cart, String> = async {
            let cart = self.ensure_cart().await?;
            let updated = api::add_cart_line(&cart.id, variant_id, quantity).await?;
            write_cart_id(&updated.id);
            self.cart.set(Some(updated.clone()));
            Ok(updated)
        }
        .await;
        self.busy.set(false);
        if let Err(err) = &result {
            self.error.set(Some(err.clone()));
        }
        result
    }
}

#[must_use]
pub fn use_cart() -> CartCtx {
    use_context::<CartCtx>().expect("CartCtx provided")
}
