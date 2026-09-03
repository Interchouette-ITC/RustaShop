# RustaShop developer targets

SHELL := /bin/bash
ROOT := $(abspath .)
CARGO ?= cargo
CLIPPY_FLAGS := -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery
API_BIND ?= 127.0.0.1:8080
DATABASE_URL ?= postgres://rustashop:rustashop@127.0.0.1:5432/rustashop

.DEFAULT_GOAL := help

.PHONY: help check test lint format format-check run-api clean db-up db-down db-psql db-wait db-migrate db-migrate-seaorm db-seed db-reset

SEAORM_PACKAGES := -p rustashop-persist -p rustashop-api
SEAORM_FEATURES := --no-default-features --features persist-seaorm

help:
	@echo "RustaShop targets"
	@echo ""
	@echo "  make check      cargo check --workspace, then SeaORM features"
	@echo "  make test       cargo test --workspace, then SeaORM feature tests"
	@echo "  make lint       fmt check + clippy (workspace + SeaORM features)"
	@echo "  make format     cargo fmt"
	@echo "  make run-api    start Actix API (RUSTASHOP_BIND, default $(API_BIND))"
	@echo "  make db-up      start Postgres via docker compose"
	@echo "  make db-down    stop Postgres container"
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

lint: format-check
	cd $(ROOT) && $(CARGO) clippy --workspace --all-targets -- $(CLIPPY_FLAGS)
	cd $(ROOT) && $(CARGO) clippy $(SEAORM_PACKAGES) --all-targets $(SEAORM_FEATURES) -- $(CLIPPY_FLAGS)

run-api:
	cd $(ROOT) && DATABASE_URL=$(DATABASE_URL) RUSTASHOP_BIND=$${RUSTASHOP_BIND:-$(API_BIND)} $(CARGO) run -p rustashop-api

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
