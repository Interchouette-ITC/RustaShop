use leptos::prelude::*;
use leptos_router::components::A;

use crate::api::Cart;
use crate::cart::use_cart;

/// Cart snapshot from the Commerce API (shared `rs.cartId` with Angular).
#[component]
pub fn CartPage() -> impl IntoView {
    let cart_ctx = use_cart();

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let _ = cart_ctx.refresh().await;
        });
    });

    view! {
        <section class="shop" aria-labelledby="cart-heading">
            <header class="shop__hero">
                <h1 id="cart-heading" class="shop__title">"Cart"</h1>
                <p class="shop__tagline">"Shared cart id with the Angular shop when both use this browser."</p>
            </header>
            <p class="shop__tagline">
                <A href="/">"Continue shopping"</A>
            </p>
            {move || cart_body(cart_ctx.error.get(), cart_ctx.busy.get(), cart_ctx.cart.get())}
        </section>
    }
}

fn cart_body(error: Option<String>, busy: bool, cart: Option<Cart>) -> AnyView {
    if let Some(message) = error {
        return view! { <p class="shop__error" role="alert">{message}</p> }.into_any();
    }
    if busy && cart.is_none() {
        return view! { <p class="shop__tagline">"Loading cart…"</p> }.into_any();
    }
    let Some(cart) = cart else {
        return view! { <p class="shop__tagline">"Your cart is empty."</p> }.into_any();
    };
    if cart.lines.is_empty() {
        return view! { <p class="shop__tagline">"Your cart is empty."</p> }.into_any();
    }
    let total = cart.items_total.display();
    let currency = cart.currency_code().to_owned();
    view! {
        <ul class="shop__cart-lines">
            <For
                each=move || cart.lines.clone()
                key=|line| format!("{}:{}", line.id, line.variant_ref())
                children=move |line| {
                    view! {
                        <li class="shop__cart-line">
                            <strong>{line.product_name}</strong>
                            <span class="shop__tagline">{line.variant_sku}</span>
                            <span>{format!("× {}", line.quantity)}</span>
                            <span>{line.unit_price.display()}</span>
                            <span>{line.line_total.display()}</span>
                        </li>
                    }
                }
            />
        </ul>
        <p class="shop__title">{format!("Total: {total} ({currency})")}</p>
    }
    .into_any()
}
