# ADRScope Makefile
# Rust project build automation

.PHONY: all build release test lint fmt fmt-check doc clean ci coverage deny check install help

# Default target
all: check

# Build targets
build:
	cargo build

release:
	cargo build --release

# Test targets
test:
	cargo test --features testing

test-verbose:
	cargo test --features testing -- --nocapture

# Linting and formatting
lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

# Documentation
doc:
	cargo doc --no-deps

doc-open:
	cargo doc --no-deps --open

# Code coverage
coverage:
	cargo tarpaulin --features testing --skip-clean --out Stdout

coverage-html:
	cargo tarpaulin --features testing --skip-clean --out Html

# Security and dependency checks
deny:
	cargo deny check

audit:
	cargo audit

# Combined check target (fast feedback)
check: fmt-check lint test

# Full CI pipeline
ci: fmt-check lint test doc deny
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
	@echo "  build        - Build debug binary"
	@echo "  release      - Build release binary"
	@echo "  test         - Run all tests"
	@echo "  test-verbose - Run tests with output"
	@echo "  lint         - Run clippy linter"
	@echo "  fmt          - Format code"
	@echo "  fmt-check    - Check code formatting"
	@echo "  doc          - Generate documentation"
	@echo "  doc-open     - Generate and open documentation"
	@echo "  coverage     - Run code coverage"
	@echo "  coverage-html- Generate HTML coverage report"
	@echo "  deny         - Run cargo-deny security checks"
	@echo "  audit        - Run cargo-audit vulnerability scan"
	@echo "  check        - Quick check (fmt + lint + test)"
	@echo "  ci           - Full CI pipeline"
	@echo "  install      - Install binary to ~/.cargo/bin"
	@echo "  clean        - Clean build artifacts"
	@echo "  setup        - Install dev dependencies"
	@echo "  help         - Show this help"
