# Contributing to rustashop

Thank you for improving rustashop. This repo is the **commerce product**. Framework work belongs in [Serenade](https://github.com/Interchouette-ITC/Serenade).

## Before you open a PR

1. Read [`ARCHITECTURE.md`](ARCHITECTURE.md) and [`docs-dev/`](../docs-dev/).
2. Run:

```bash
make lint
make test
```

Integration tests need Postgres (`make db-up`). Prefer Docker for the database.

3. One concern per PR. Prefer draft until the slice is complete.
4. English only in code, docs, commits, and PR text. In markdown prose, write
   **Serenade** (capital S); crate ids stay lowercase (`serenade-contracts`).

## Toolchain

- Rust stable (see `rust-version` in the workspace `Cargo.toml`).
- Integration branch: `dev` on the org repo.
- Feature branches land via PR from the worker fork.

## Make targets

| Target | Purpose |
| --- | --- |
| `make lint` | `fmt --check` + clippy (`-D warnings`, pedantic, nursery) for workspace and SeaORM features |
| `make test` | workspace tests, then SeaORM feature tests |
| `make doc` | rustdoc (`-D warnings`) |
| `make openapi` | write `openapi/openapi.json` from utoipa |
| `make shop-angular` | serve Angular shop (`shops/angular`, port 4242) |
| `make admin-angular` | serve Angular admin (`admin/angular`, port 4250) |
| `make shop-leptos-rangular` | serve Leptos+rangular shop (`shops/leptos-rangular`, port 4181) |
| `make run-api` | Actix API on host (`RUSTASHOP_BIND`, default `127.0.0.1:8080`) |
| `make db-up` | Postgres only via compose |
| `make stack-up` | Postgres + migrate + API image |
| `make db-migrate` | SQLx migrations |
| `make db-migrate-seaorm` | SeaORM migrations |
| `make db-seed` | catalog seed SQL (idempotent; does not wipe) |
| `make db-reset` | **DESTROYS** schema `public` then migrates; requires `CONFIRM=YES` |

Shared shop markup/SCSS: `templates/default/`. Admin markup/SCSS: `templates/default-admin/`.
Do not edit generated adapters under `shops/*/generated/` or `admin/*/generated/`
(build output, gitignored).

Default DSN: `postgres://rustashop:rustashop@127.0.0.1:5432/rustashop`.

## Quality bar

Do not add `#[allow(clippy::too_many_arguments)]`, `too_many_lines`, or `dead_code`. Fix with structs, helpers, or by wiring/removing unused items.

## Persistence features

Default build uses `persist-sqlx`. SeaORM path:

```bash
cargo check -p rustashop-persist -p rustashop-api --no-default-features --features persist-seaorm
```

`make lint` and `make test` already cover both.

## Documentation

- **Product architecture:** [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **Foundations** (Wasm, realtime, AI, domains): [`docs-dev/`](../docs-dev/)
- **OpenAPI:** live at `/openapi.json` and `/swagger-ui/`; committed dump via `make openapi`
- No plan jargon or host-absolute paths in shipped text

## Issues and epics

- Commerce and product ops: this repo’s GitHub issues / milestones.
- Framework kernel, DI, HTTP foundation, console: Serenade issues.
- Kernel wire into rustashop: [#49](https://github.com/Interchouette-ITC/rustashop/issues/49).

## Commits and PRs

Conventional commits (`feat:`, `fix:`, `docs:`, `ci:`, …). PR body: **Summary** + **Test plan** only.

## License

This repository is licensed under **OSL-3.0** (see [`../LICENSE`](../LICENSE)).

## Questions

Open a GitHub issue on `Interchouette-ITC/rustashop` for product design. Framework questions that affect multiple apps go to Serenade.
