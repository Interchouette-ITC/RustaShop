//! rustashop Leptos CSR shop (track B).

// Leptos `#[component]` TypedBuilder emits empty marker enums; nursery lint.
#![allow(clippy::empty_enums)]

use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
