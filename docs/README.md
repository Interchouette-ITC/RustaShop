# rustashop

<p align="center">
  <img src="brand/wordmark-gh-dark.svg#gh-dark-mode-only" alt="rustashop" height="56" />
  <img src="brand/wordmark-gh-light.svg#gh-light-mode-only" alt="rustashop" height="56" />
</p>

<p align="center">
  <img src="brand/logo-mascot-readme.png" alt="rustashop mascot: Ferris in a shopping bag with Rust gear" width="320" />
</p>

<p align="center">
  <strong>Modern commerce. Rust powered. AI native.</strong>
</p>

One **Rust** commerce API. **Angular** or **rangular** clients on the same OpenAPI contracts. Shared storefront markup lives in `templates/default/`; each shop host adapts it. rangular targets **two renderers**: Leptos (web/DOM) and GPUI (native GPU). See [`../docs-dev/UI-RENDERERS.md`](../docs-dev/UI-RENDERERS.md).

AI is on the product map (discovery, shopping agents, catalog assist, pricing, support, MCP), not glued on later. See [`../docs-dev/AI-NATIVE.md`](../docs-dev/AI-NATIVE.md).

**Status today:** catalog + cart + checkout HTTP, OpenAPI / Swagger UI, local Postgres (Docker), two shop hosts on shared templates, and an Angular admin sample (orders list + status PATCH). Payments and realtime are next.

## What you get today

| Piece            | Role                                                                                                                                                |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Commerce API** | Actix-web: products, carts, checkout → orders; money as integers                                                                                    |
| **OpenAPI**      | utoipa + Swagger UI at `/swagger-ui/`; `make openapi` writes `openapi/openapi.json`                                                                 |
| **Persistence**  | Postgres; SQLx default, SeaORM feature path; Docker compose                                                                                         |
| **Templates**    | `templates/default/` (shops) and `templates/default-admin/` (operator BO)                                                                           |
| **UI A**         | `shops/angular` - Angular storefront (catalog, cart, checkout)                                                                                      |
| **UI B**         | `shops/leptos-rangular` - Leptos + rangular (catalog, product, cart)                                                                                |
| **Admin**        | `admin/angular` - Angular sample BO (orders table + status PATCH; bearer token)                                                                     |
| **Framework**    | [Serenade](https://github.com/Interchouette-ITC/Serenade) contracts / kernel wire ([#49](https://github.com/Interchouette-ITC/rustashop/issues/49)) |

Still building toward: one payment provider, WebSocket live surfaces, MCP / agent tools (Axum).

## Quick start

```bash
git clone https://github.com/Interchouette-ITC/rustashop.git
cd rustashop
make help
```

### API + database

```bash
make db-up && make db-migrate && make db-seed
make run-api
# or all-in-one: make stack-up

curl http://127.0.0.1:8080/healthz
curl http://127.0.0.1:8080/v1/products
```

Swagger UI: `http://127.0.0.1:8080/swagger-ui/`. Do not bind port `8080` twice.

### Shop fronts (API already running)

```bash
make shop-angular            # http://127.0.0.1:4242/
make admin-angular           # http://127.0.0.1:4250/ (paste RUSTASHOP_ADMIN_API_TOKEN)
make shop-leptos-rangular    # http://127.0.0.1:4181/
```

Designers edit `templates/default/` (shop) and `templates/default-admin/` (admin). Do not
hand-edit `shops/*/generated/` or `admin/*/generated/` (build output, gitignored).

### Quality gate

```bash
make lint
make test
```

## Domains

| Host                           | Role                                    |
| ------------------------------ | --------------------------------------- |
| `rustashop.interchouette.net`  | First `:dev` image tip (operator-owned) |
| `rustashop.ai`                 | Primary marketing                       |
| `rustashop.io`                 | Product-oriented                        |
| `rustashop.dev`                | Demo                                    |
| `rustashop.app`                | Ionic app                               |
| `rustashop.nl` / `.eu` / `.fr` | Redirect → `.ai` for now                |

Detail: [`../docs-dev/DOMAINS.md`](../docs-dev/DOMAINS.md).

## Docs

| Doc                                              | Topic                                     |
| ------------------------------------------------ | ----------------------------------------- |
| [`ARCHITECTURE.md`](ARCHITECTURE.md)             | Crates, HTTP split, request path          |
| [`CONTRIBUTING.md`](CONTRIBUTING.md)             | Make targets, lint bar, PR habits         |
| [`../docs-dev/README.md`](../docs-dev/README.md) | Foundations (Wasm, realtime, AI, domains) |
| [`brand/`](brand/)                               | Brand assets                              |

**rangular** is upstream UI tooling. Template language changes belong there; commerce domain belongs here.

## Contributing

1. Read [`ARCHITECTURE.md`](ARCHITECTURE.md) and [`../docs-dev/`](../docs-dev/).
2. Follow [`CONTRIBUTING.md`](CONTRIBUTING.md) (`make lint`, `make test`).
3. One concern per PR. Commits and docs in **English**.

<p align="center">
  <img src="brand/badge-stack-readme.png" alt="rustashop stack: Rust API, Angular, rangular" width="480" style="margin-top: 1.5rem; margin-bottom: 0.25rem;" />
</p>

## Thanks

**rustashop** stands on excellent open-source projects and hosts:

| Project | Role here |
| --- | --- |
| [Rust](https://www.rust-lang.org/) | Commerce API, workers, and Wasm shop host |
| [Tokio](https://tokio.rs/) | Async runtime |
| [Actix Web](https://actix.rs/) | Commerce HTTP API, OpenAPI, Swagger UI |
| [utoipa](https://github.com/juhaku/utoipa) | OpenAPI types and `/swagger-ui/` |
| [SQLx](https://github.com/launchbadge/sqlx) | Default Postgres persistence (no ORM) |
| [SeaORM](https://www.sea-ql.org/SeaORM/) | Alternate ORM persistence path |
| [PostgreSQL](https://www.postgresql.org/) | System of record |
| [Angular](https://angular.dev/) | UI track A storefront |
| [Leptos](https://leptos.dev/) | UI track B web renderer (CSR / Trunk) |
| [rangular](https://github.com/Interchouette-ITC/rangular) | Shared Angular-shaped templates → Leptos |
| [Serenade](https://github.com/Interchouette-ITC/Serenade) | Application framework / contracts |
| [Axum](https://github.com/tokio-rs/axum) | MCP / agent HTTP surfaces (next) |
| [Render](https://render.com/) | Hosting tip / demos (operator-owned) |

Thank you to their maintainers and communities.

## License

**OSL-3.0** (Open Software License v3.0). See [`LICENSE`](../LICENSE).

<p align="center">
  <img src="brand/seal-crab-128.png" alt="rustashop seal: crab" width="128" height="128" style="margin-top: 1.25rem; margin-bottom: 0; vertical-align: middle;" />
</p>
