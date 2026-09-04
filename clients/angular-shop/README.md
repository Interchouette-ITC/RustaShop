# Angular shop (UI track A) — `@rustashop/shop-angular` 0.1.0

Customer **shop** SPA for RustaShop. Same Commerce API as the Leptos+rangular shop.

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
- HTML / router base: default `/` (`angular.json`). Override: `RUSTASHOP_BASE_HREF=/shop/ make shop-angular`, or `ng serve --base-href /shop/` / `ng build --base-href /shop/`.
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
