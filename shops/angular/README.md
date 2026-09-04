# Angular shop (UI track A) - `@rustashop/shop-angular` 0.1.0

Customer **shop** SPA for rustashop against the Commerce API.

## CSS / JS ownership

Front-end styling and shop JS live in **this package** (npm dependencies + `src/styles/`).

| Concern                | Owner                                                         |
| ---------------------- | ------------------------------------------------------------- |
| Bootstrap CSS (themed) | Angular shop (`bootstrap` npm + `src/styles/_bootstrap.scss`) |
| Brand tokens           | Angular shop (`src/styles/_tokens.scss`)                      |
| Interactive UI         | Angular components (no Bootstrap JS CDN)                      |
| Commerce API           | CSS-agnostic; no shop CSS/JS ownership                        |

No CDN tags in `index.html`. The SPA owns its CSS/JS bundles (same role as a Webpack Encore-style frontend package).

## Layout (Nx-ready folders + path aliases)

```text
src/app/
  api/           # OpenAPI types + domain HTTP clients → @rustashop/shop-api
                 #   models.ts, api-client.ts, catalog|cart|checkout|health.api.ts
  core/          # CatalogStore, CartStore, CheckoutService (signals)
  shared/        # shell, pipes, ui (ProductCard) → @rustashop/shop-shared
  features/      # catalog, cart, checkout pages (lazy routes)
```

No Nx yet: path aliases (`tsconfig.json`) mirror future libs. Pages stay thin and bind store signals.

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
