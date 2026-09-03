# Architecture

RustaShop is a Rust commerce product. The [Serenade](https://github.com/Interchouette-ITC/Serenade) framework supplies kernel concepts (DI, events, config, contracts). This repo owns commerce domain, persistence adapters, and HTTP surfaces.

## Layers

```text
Clients (Angular | rangular)
        │  OpenAPI + (later) WebSocket
        ▼
rustashop-api (Actix)     rustashop-mcp (Axum, placeholder)
        │
        ▼
rustashop-domain          pure types (Money, Product, …)
        │
        ▼
rustashop-persist         feature-selected facade
   ┌────┴────┐
sqlx        seaorm
        │
        ▼
PostgreSQL
```

Serenade kernel wire (`rustashop` crate) lands when framework HTTP + bundles are ready. Until then the app boots Actix directly with persist + domain.

## Crates (today)

| Crate | Role |
| --- | --- |
| `rustashop` | App kernel placeholder until Serenade path/git wire |
| `rustashop-domain` | Money, Product, Variant, Category (no ORM types) |
| `rustashop-persist` | Facade: `persist-sqlx` (default) or `persist-seaorm` |
| `rustashop-persist-sqlx` | SQLx migrations, catalog repos, migrate binary |
| `rustashop-persist-seaorm` | SeaORM mirror schema and catalog repos |
| `rustashop-api` | Actix commerce HTTP, OpenAPI, Swagger UI |
| `rustashop-mcp` | Axum MCP / agent tools (placeholder) |

## HTTP house split

| Surface | Framework | Owns |
| --- | --- | --- |
| Commerce API | **Actix-web** | Catalog, cart, checkout, orders, webhooks, admin REST, WebSocket gateway (later) |
| MCP / tools | **Axum** | Streamable MCP and narrow agent endpoints |

Both share domain and persist. OpenAPI is generated with **utoipa** on the Actix crate (`/openapi.json`, `/swagger-ui/`). Regenerated file: `openapi/openapi.json` via `make openapi`.

## Request path (catalog today)

```text
GET /v1/products
  → rustashop-api handler
  → ProductRepository (serenade-contracts)
  → SqlxCatalogRepository | SeaOrmCatalogRepository
  → PostgreSQL
```

Intended commerce path when cart/checkout land: catalog → cart → checkout → order (same stack; messenger/events via Serenade when wired).

## Persistence

- Postgres in Docker (`docker compose`); no host Postgres install.
- Dual backends behind one facade; enable exactly one of `persist-sqlx` / `persist-seaorm`.
- Diesel is deferred (separate issue).
- Repository traits come from **serenade-contracts**; adapters live here.

## Room for later crates

Leave headroom for:

| Lane | Intent |
| --- | --- |
| Realtime | WebSocket gateway aligned with OpenAPI mutations |
| Extensions | WIT / Component Model host hooks |
| Sandbox | Wasmer (or similar) for untrusted / polyglot scripts |

Wasm roles (UI wasm vs plugins vs sandbox) are spelled out in [`docs-dev/WASM-LAYERS.md`](../docs-dev/WASM-LAYERS.md). Foundations overview: [`docs-dev/FOUNDATIONS.md`](../docs-dev/FOUNDATIONS.md).

## Local run

| Mode | Command |
| --- | --- |
| Full stack | `docker compose up --build` (Postgres + migrate + API on `8080`) |
| Host API | `make db-up && make db-migrate && make run-api` |

Do not bind `8080` twice. Details: [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Related

- Framework: [Serenade](https://github.com/Interchouette-ITC/Serenade)
- App kernel wire: issue [#49](https://github.com/Interchouette-ITC/RustaShop/issues/49)
- Contributor docs epic: [#10](https://github.com/Interchouette-ITC/RustaShop/issues/10)
