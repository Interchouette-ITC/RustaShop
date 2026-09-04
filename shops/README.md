# Shops (customer UI hosts)

HTTP UIs that speak the Commerce API. Controllers live here; designers edit
`templates/` only.

| Path | Make | Role |
| --- | --- | --- |
| `angular/` | `make shop-angular` | Track A (Angular SPA) |
| `leptos-rangular/` | `make shop-leptos-rangular` | Track B (Leptos + rangular; port `4181`) |

These are **shop hosts**, not Serenade bundles and not the theme. Themes:
`templates/<theme>/`. Multi-shop / white-label later = more themes (and later
more shop hosts), not more copies of markup inside each host.
