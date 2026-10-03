# ============================================================
# textstat — dev workflow (run from inside textstat/)
# ============================================================
.PHONY: help fmt fmt-check lint test check run build clean

fmt:
	cargo fmt --manifest-path Cargo.toml

fmt-check:
	cargo fmt --manifest-path Cargo.toml --check

lint:
	cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings

test:
	cargo test --manifest-path Cargo.toml

check: fmt-check lint test

run: check
	cargo run --manifest-path Cargo.toml

build:
	cargo build --manifest-path Cargo.toml

clean:
	cargo clean --manifest-path Cargo.toml

help:
	@echo "textstat: make fmt | fmt-check | lint | test | check | run | build | clean"
