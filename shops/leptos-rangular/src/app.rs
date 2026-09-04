use leptos::prelude::*;

use crate::components::ProductCardPanel;

struct SampleProduct {
    name: &'static str,
    slug: &'static str,
    description: Option<&'static str>,
    detail_href: &'static str,
}

const SAMPLES: &[SampleProduct] = &[
    SampleProduct {
        name: "Demo Mug",
        slug: "demo-mug",
        description: Some("Shared theme card (same HTML/SCSS as Angular)."),
        detail_href: "/products/1",
    },
    SampleProduct {
        name: "Demo Tee",
        slug: "demo-tee",
        description: None,
        detail_href: "/products/2",
    },
];

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="shop">
            <header class="shop__hero">
                <h1 class="shop__title">"rustashop"</h1>
                <p class="shop__tagline">
                    "Leptos host + shared templates/ theme (track B)"
                </p>
            </header>
            <section class="shop__catalog" aria-label="Sample catalog">
                {SAMPLES
                    .iter()
                    .map(|product| {
                        view! {
                            <ProductCardPanel
                                name=product.name.to_owned()
                                slug=product.slug.to_owned()
                                description=product.description.map(str::to_owned)
                                detail_href=product.detail_href.to_owned()
                            />
                        }
                    })
                    .collect_view()}
            </section>
        </main>
    }
}
