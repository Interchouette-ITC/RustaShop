# Leptos + rangular shop (track B)

Customer shop host: **Leptos** CSR (Trunk / wasm) with **rangular** controllers and
shared template files under `templates/<id>/`. Same Commerce API as the Angular shop.

## Run

From the repo root:

```bash
make shop-leptos-rangular
# → http://127.0.0.1:4181/
```

Release build:

```bash
cd shops/leptos-rangular && trunk build --release
```

Port override: `make shop-leptos-rangular SHOP_LEPTOS_PORT=3000`.

## Layout

| Path | Role |
| --- | --- |
| `../../templates/<id>/` | Shared `.html` / `.scss` |
| `src/components/<name>/` | Rust controllers |
| `style/main.css` | Host-only chrome (temporary catalog page) |
| `generated/components.css` | From `build.rs` (template SCSS) |
| `build.rs` | rangular AOT + SCSS compile |

rangular crates come from git `Interchouette-ITC/rangular` branch `dev`.
