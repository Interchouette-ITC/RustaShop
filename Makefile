# RustaShop developer targets

SHELL := /bin/bash
ROOT := $(abspath .)
CARGO ?= cargo
CLIPPY_FLAGS := -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery
API_BIND ?= 127.0.0.1:8080
DATABASE_URL ?= postgres://rustashop:rustashop@127.0.0.1:5432/rustashop

.DEFAULT_GOAL := help

.PHONY: help check test lint format format-check run-api clean db-up db-down db-psql db-wait db-migrate db-reset

help:
	@echo "RustaShop targets"
	@echo ""
	@echo "  make check      cargo check --workspace"
	@echo "  make test       cargo test --workspace"
	@echo "  make lint       fmt check + clippy (workspace)"
	@echo "  make format     cargo fmt"
	@echo "  make run-api    start Actix API (RUSTASHOP_BIND, default $(API_BIND))"
	@echo "  make db-up      start Postgres via docker compose"
	@echo "  make db-down    stop Postgres container"
	@echo "  make db-psql    psql shell (needs db-up)"
	@echo "  make db-migrate run SQLx migrations (needs db-up, DATABASE_URL)"
	@echo "  make db-reset   drop public schema and re-run migrations"
	@echo "  make clean      cargo clean"
	@echo ""
	@echo "Overrides: API_BIND=$(API_BIND)"

check:
	cd $(ROOT) && $(CARGO) check --workspace

test:
	cd $(ROOT) && $(CARGO) test --workspace

format:
	cd $(ROOT) && $(CARGO) fmt

format-check:
	cd $(ROOT) && $(CARGO) fmt --check

lint: format-check
	cd $(ROOT) && $(CARGO) clippy --workspace --all-targets -- $(CLIPPY_FLAGS)

run-api:
	cd $(ROOT) && RUSTASHOP_BIND=$${RUSTASHOP_BIND:-$(API_BIND)} $(CARGO) run -p rustashop-api

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

db-reset:
	cd $(ROOT) && docker compose exec -T postgres psql -U rustashop -d rustashop -c 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;'
	@$(MAKE) db-migrate

clean:
	cd $(ROOT) && $(CARGO) clean
