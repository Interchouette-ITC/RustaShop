# Themes (designer surface)

Shared markup and component styles for shop hosts. Controllers stay under
`shops/` (`.ts` / `.rs`); designers edit only this tree.

```text
templates/<theme>/<component>/<component>.html
templates/<theme>/<component>/<component>.scss
```

Default theme: `default/`.

## Naming (MVC-like)

Component id = directory stem = file stem (snake_case), same on both hosts:

| Layer | Example |
| --- | --- |
| Theme | `templates/default/product_card/product_card.{html,scss}` |
| Leptos Host | `shops/leptos-rangular/src/components/product_card/` |
| Angular controller | `shops/angular/.../product-card.ts` (kebab file OK) → loads `product_card` |

Do not rename the theme stem without renaming the Host module. Angular may use
kebab filenames; the **theme id** stays snake_case for rangular.

Markup must stay in the rangular subset. No `routerLink`, no signal-call
syntax (`name()`), no framework-only directives.

| Consumer | How it loads files |
| --- | --- |
| `shops/angular` | `templateUrl` + SCSS `@use` via `includePaths` → `../../templates/default` |
| `shops/leptos-rangular` | `build.rs` reads `../../templates/<theme>` (`RUSTASHOP_THEME`) |

## Vs PHP cousins

| PrestaShop / Sylius | rustashop |
| --- | --- |
| `themes/<name>/` | `templates/<theme>/` |
| Controllers / modules | `shops/<host>/` (+ later admin hosts) |
| Bundles / modules (features) | Rust crates / Serenade bundles (not this tree) |

A theme is presentation. A shop host is a renderer stack. A bundle/crate is
domain or framework wiring. Keep those three separate so white-label themes and
extra shop hosts do not fork controllers or API crates.
