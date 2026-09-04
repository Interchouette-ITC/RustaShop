use leptos::prelude::*;
use leptos_router::components::A;

use crate::cart::use_cart;

/// Leptos shop chrome (nav + cart badge). Markup/SCSS for Angular `shop_shell` live in `templates/default`.
#[component]
pub fn ShopShell(children: Children) -> impl IntoView {
    let cart = use_cart();

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let _ = cart.ensure_cart().await;
        });
    });

    view! {
        <div class="shell">
            <header class="shell__bar">
                <div class="shell__inner">
                    <A href="/" attr:class="shell__brand">
                        <span class="shell__name">"rustashop"</span>
                    </A>
                    <nav class="shell__nav" aria-label="Shop">
                        <A href="/" attr:class="">
                            "Catalog"
                        </A>
                        <A href="/cart" attr:class="">
                            "Cart"
                            {move || {
                                let count = cart.line_count();
                                (count > 0).then(|| view! {
                                    <span class="shell__badge">" " {count}</span>
                                })
                            }}
                        </A>
                    </nav>
                </div>
            </header>
            <main class="shell__main">
                {children()}
            </main>
        </div>
    }
}
