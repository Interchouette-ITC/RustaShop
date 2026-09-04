# Angular shop (UI track A)

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

Dev server: `http://127.0.0.1:4242/` (proxy `/api` → Actix via `proxy.conf.js`, default `:8080`).

## OpenAPI client

Types are generated from the committed `openapi/openapi.json`:

```bash
npm run generate:api
```

Refresh the dump from Rust with `make openapi` at the repo root, then regenerate.

## Build

```bash
npm run build
```
