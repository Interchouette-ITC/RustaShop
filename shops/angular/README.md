# Angular shop (UI track A) - `@rustashop/shop-angular` 0.1.0

Customer **shop** SPA for rustashop against the Commerce API.

## CSS / JS ownership

Designers edit **`templates/default`**. This host only owns controllers and the
`bootstrap` npm dep that Sass resolves when compiling the template package.

| Concern              | Owner                                                             |
| -------------------- | ----------------------------------------------------------------- |
| Bootstrap + tokens   | `@rustashop/template-default` (`bootstrap.scss`, `tokens.scss`)   |
| Global chrome        | `@rustashop/template-default` (`shop.scss`)                       |
| Component markup/CSS | `@rustashop/template-default` (`<id>/<id>.html` + `.scss`)        |
| Interactive UI       | Angular controllers under `src/components/` (no Bootstrap JS CDN) |
| Commerce API         | CSS-agnostic                                                      |

No CDN tags in `index.html`. Controllers use `@generated/<id>.ng` (from
`npm run template:emit` → `scripts/emit-template.mjs`).

## Layout

```text
src/
  app/
    api/           # OpenAPI types + domain HTTP clients
    core/          # CatalogStore, CartStore, CheckoutService (signals)
    shared/        # shared pipes / helpers
  components/      # page controllers (shop_shell, product_*, cart_page, checkout_page)
  environments/    # apiBaseUrl and build flags
generated/         # template emit + OpenAPI client (gitignored; do not hand-edit)
```

Controllers import `@generated/<id>.ng` from `npm run template:emit`. Pages stay thin and bind store signals.

## Prerequisites

1. Postgres + migrations + seed: `make db-up && make db-migrate && make db-seed` (from repo root).
2. API on `127.0.0.1:8080`: `make run-api` (or `make stack-up`). If another process owns `:8080`, free it or use `API_BIND=127.0.0.1:8081` and `RUSTASHOP_API_PROXY=http://127.0.0.1:8081`.

## Develop

From the repo root:

```bash
make shop-angular
FORCE=1 make shop-angular   # re-run npm install
```

Or here:

```bash
npm install
npm run generate:api
npm start
```

- Dev server host/port: `angular.json` (`127.0.0.1:4242`), not `package.json`.
- App API base URL: `src/environments/` (`apiBaseUrl`, default `/api` via proxy).
- HTML / router base: default `/` (`angular.json` `baseHref`, `<base href>` in `index.html`).
  - Production / static: `ng build --base-href /shop/`.
  - Dev serve under a path: `RUSTASHOP_BASE_HREF=/shop/ make shop-angular` passes `--serve-path` (Angular CLI no longer accepts `--base-href` on `ng serve`).
- Unit tests: Vitest via `@angular/build:unit-test` (`npm test`).
- Lint / format: `npm run lint`, `npm run format` / `npm run format:check`.

## OpenAPI client

```bash
npm run generate:api
```

Refresh the dump from Rust with `make openapi` at the repo root, then regenerate.

## Build

```bash
npm run build
ng build --base-href /shop/
```
