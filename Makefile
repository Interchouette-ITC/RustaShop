# RustaShop developer targets

SHELL := /bin/bash
ROOT := $(abspath .)
CARGO ?= cargo
CLIPPY_FLAGS := -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery
RUSTDOCFLAGS ?= -D warnings
API_BIND ?= 127.0.0.1:8080
DATABASE_URL ?= postgres://rustashop:rustashop@127.0.0.1:5432/rustashop
OPENAPI_OUT ?= openapi/openapi.json
SHOP_ANGULAR_DIR := clients/angular-shop
SHOP_ANGULAR_PORT ?= 4242

.DEFAULT_GOAL := help

.PHONY: help check test lint format format-check check-sql-safety doc doc-open openapi run-api clean db-up db-down db-psql db-wait db-migrate db-migrate-seaorm db-seed db-reset stack-up shop-angular shop-leptos-rangular

SEAORM_PACKAGES := -p rustashop-persist -p rustashop-api
SEAORM_FEATURES := --no-default-features --features persist-seaorm

help:
	@echo "RustaShop targets"
	@echo ""
	@echo "  make check      cargo check --workspace, then SeaORM features"
	@echo "  make test       cargo test --workspace, then SeaORM feature tests"
	@echo "  make lint       fmt check + SQL safety test + clippy (workspace + SeaORM features)"
	@echo "  make check-sql-safety  cargo test: deny format!-built SQL in persist crates"
	@echo "  make doc        rustdoc for all crates (-D warnings)"
	@echo "  make doc-open   build docs and open in browser"
	@echo "  make openapi    write $(OPENAPI_OUT) from utoipa"
	@echo "  make shop-angular  serve Angular shop ($(SHOP_ANGULAR_DIR), port $(SHOP_ANGULAR_PORT))"
	@echo "  make shop-leptos-rangular  serve Leptos+rangular shop (when client lands)"
	@echo "  make format     cargo fmt"
	@echo "  make run-api    start Actix API on the host (RUSTASHOP_BIND, default $(API_BIND))"
	@echo "  make db-up      start Postgres via docker compose"
	@echo "  make db-down    stop the compose project (Postgres and API if started)"
	@echo "  make stack-up   build and start Postgres + migrate + API"
	@echo "  make db-psql    psql shell (needs db-up)"
	@echo "  make db-migrate run SQLx migrations (needs db-up, DATABASE_URL)"
	@echo "  make db-migrate-seaorm run SeaORM migrations (needs db-up, DATABASE_URL)"
	@echo "  make db-seed    load catalog seed (needs db-up, migrated schema)"
	@echo "  make db-reset   drop public schema and re-run migrations"
	@echo "  make clean      cargo clean"
	@echo ""
	@echo "Overrides: API_BIND=$(API_BIND)"

check:
	cd $(ROOT) && $(CARGO) check --workspace
	cd $(ROOT) && $(CARGO) check $(SEAORM_PACKAGES) $(SEAORM_FEATURES)

test:
	cd $(ROOT) && $(CARGO) test --workspace
	cd $(ROOT) && $(CARGO) test $(SEAORM_PACKAGES) $(SEAORM_FEATURES)

format:
	cd $(ROOT) && $(CARGO) fmt

format-check:
	cd $(ROOT) && $(CARGO) fmt --check

check-sql-safety:
	cd $(ROOT) && $(CARGO) test -p rustashop-persist-sqlx --test sql_safety

lint: format-check check-sql-safety
	cd $(ROOT) && $(CARGO) clippy --workspace --all-targets -- $(CLIPPY_FLAGS)
	cd $(ROOT) && $(CARGO) clippy $(SEAORM_PACKAGES) --all-targets $(SEAORM_FEATURES) -- $(CLIPPY_FLAGS)

doc:
	cd $(ROOT) && RUSTDOCFLAGS="$(RUSTDOCFLAGS)" $(CARGO) doc --workspace --no-deps

doc-open: doc
	cd $(ROOT) && RUSTDOCFLAGS="$(RUSTDOCFLAGS)" $(CARGO) doc --workspace --no-deps --open

openapi:
	cd $(ROOT) && $(CARGO) run -p rustashop-api --bin rustashop-openapi -- $(OPENAPI_OUT)

shop-angular:
	@if ss -tln | grep -qE ':$(SHOP_ANGULAR_PORT)\\b'; then \
		echo "port $(SHOP_ANGULAR_PORT) already in use; stop the other process or set SHOP_ANGULAR_PORT="; \
		exit 1; \
	fi
	cd $(ROOT)/$(SHOP_ANGULAR_DIR) && \
		if [ ! -d node_modules ]; then npm install; fi && \
		npm run generate:api && \
		npm start -- --port $(SHOP_ANGULAR_PORT)

shop-leptos-rangular:
	@echo "clients/leptos-rangular-shop is not scaffolded yet (see GitHub #23)"
	@exit 1

run-api:
	cd $(ROOT) && DATABASE_URL=$(DATABASE_URL) RUSTASHOP_BIND=$${RUSTASHOP_BIND:-$(API_BIND)} $(CARGO) run -p rustashop-api

stack-up:
	cd $(ROOT) && docker compose up --build -d

db-up:
	cd $(ROOT) && docker compose up -d postgres
	@$(MAKE) db-wait

db-down:
	cd $(ROOT) && docker compose down

db-wait:
	@cd $(ROOT) && docker compose exec -T postgres sh -c 'until pg_isready -U rustashop -d rustashop; do sleep 1; done'

db-psql:
	cd $(ROOT) && docker compose exec postgres psql -U rustashop -d rustashop

db-migrate:
	cd $(ROOT) && DATABASE_URL=$(DATABASE_URL) $(CARGO) run -p rustashop-persist-sqlx --bin rustashop-migrate

db-migrate-seaorm:
	cd $(ROOT) && DATABASE_URL=$(DATABASE_URL) $(CARGO) run -p rustashop-persist-seaorm --bin rustashop-seaorm-migrate

db-seed:
	cd $(ROOT) && docker compose exec -T postgres psql -U rustashop -d rustashop < db/seeds/catalog.sql

db-reset:
	cd $(ROOT) && docker compose exec -T postgres psql -U rustashop -d rustashop -c 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;'
	@$(MAKE) db-migrate

clean:
	cd $(ROOT) && $(CARGO) clean
