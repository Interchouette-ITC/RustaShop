# RustaShop developer targets

SHELL := /bin/bash
ROOT := $(abspath .)
CARGO ?= cargo
CLIPPY_FLAGS := -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery
API_BIND ?= 127.0.0.1:8080

.DEFAULT_GOAL := help

.PHONY: help check test lint format format-check run-api clean

help:
	@echo "RustaShop targets"
	@echo ""
	@echo "  make check      cargo check --workspace"
	@echo "  make test       cargo test --workspace"
	@echo "  make lint       fmt check + clippy (workspace)"
	@echo "  make format     cargo fmt"
	@echo "  make run-api    start Actix API (RUSTASHOP_BIND, default $(API_BIND))"
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

clean:
	cd $(ROOT) && $(CARGO) clean
