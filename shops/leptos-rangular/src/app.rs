use leptos::prelude::*;

use crate::pages::CatalogPage;

/// Host root (Angular analog: `rs-root` + `<router-outlet />`).
#[component]
pub fn App() -> impl IntoView {
    view! { <CatalogPage /> }
}
