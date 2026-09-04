# Leptos + rangular shop (track B)

Customer shop host: **Leptos** CSR (Trunk / wasm) with **rangular** controllers and
shared theme files under `templates/<theme>/`. Same Commerce API as the Angular shop.

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
Theme override: `RUSTASHOP_THEME=default` (default).

## Layout

| Path | Role |
| --- | --- |
| `../../templates/<theme>/` | Shared `.html` / `.scss` (designer surface) |
| `src/components/<name>/` | Rust Host controllers (same stem as theme) |
| `build.rs` | AOT + SCSS from the theme → `style/components.generated.css` |

rangular crates come from git `Interchouette-ITC/rangular` branch `dev`.
