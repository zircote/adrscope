# ADRScope Makefile
# Rust project build automation

.PHONY: all build release test lint fmt fmt-check doc clean ci coverage deny check install help msrv

# CI environment flags (match GitHub Actions)
export RUSTFLAGS := -D warnings
export RUSTDOCFLAGS := -D warnings

# Default target
all: check

# Build targets
build:
	cargo build

release:
	cargo build --release

# Test targets
test:
	cargo test --all-features --verbose

test-quiet:
	cargo test --all-features

# Linting and formatting
lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

# Documentation
doc:
	cargo doc --no-deps --all-features

doc-open:
	cargo doc --no-deps --all-features --open

# Code coverage
coverage:
	cargo tarpaulin --all-features --skip-clean --out Stdout

coverage-html:
	cargo tarpaulin --all-features --skip-clean --out Html

# Security and dependency checks
deny:
	cargo deny check

audit:
	cargo audit

# MSRV check (Minimum Supported Rust Version)
# Requires: rustup toolchain install 1.85
msrv:
	@rustup run 1.85 rustc --version >/dev/null 2>&1 || \
		(echo "Error: Rust 1.85 not installed. Run: rustup toolchain install 1.85" && exit 1)
	rustup run 1.85 cargo check --all-features

# Combined check target (fast feedback)
check: fmt-check lint test-quiet

# Full CI pipeline (matches GitHub Actions exactly)
ci: fmt-check lint test doc deny msrv
	@echo "CI checks passed!"

# Install binary to ~/.cargo/bin
install:
	cargo install --path .

# Clean build artifacts
clean:
	cargo clean

# Install development dependencies
setup:
	rustup component add clippy rustfmt
	cargo install cargo-tarpaulin cargo-deny cargo-audit

# Help
help:
	@echo "Available targets:"
	@echo "  build         - Build debug binary"
	@echo "  release       - Build release binary"
	@echo "  test          - Run all tests (verbose)"
	@echo "  test-quiet    - Run tests (quiet output)"
	@echo "  lint          - Run clippy linter"
	@echo "  fmt           - Format code"
	@echo "  fmt-check     - Check code formatting"
	@echo "  doc           - Generate documentation"
	@echo "  doc-open      - Generate and open documentation"
	@echo "  coverage      - Run code coverage"
	@echo "  coverage-html - Generate HTML coverage report"
	@echo "  deny          - Run cargo-deny security checks"
	@echo "  audit         - Run cargo-audit vulnerability scan"
	@echo "  msrv          - Check minimum supported Rust version"
	@echo "  check         - Quick check (fmt + lint + test)"
	@echo "  ci            - Full CI pipeline (matches GitHub Actions)"
	@echo "  install       - Install binary to ~/.cargo/bin"
	@echo "  clean         - Clean build artifacts"
	@echo "  setup         - Install dev dependencies"
	@echo "  help          - Show this help"
