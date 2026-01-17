# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ADRScope is a Rust CLI tool that generates self-contained HTML viewers for Architecture Decision Records (ADRs). It parses ADRs in the [structured-MADR](https://github.com/zircote/structured-madr) format, validates them, computes statistics, and generates interactive HTML viewers with faceted search, relationship graphs, and GitHub Wiki pages.

## Build Commands

```bash
make check      # Quick check: fmt + lint + test (use for development)
make ci         # Full CI pipeline: fmt + lint + test + doc + deny + msrv
make test       # Run all tests with verbose output
cargo test test_name           # Run specific test
cargo test -- --nocapture      # Run tests with stdout visible
make lint       # Run clippy with -D warnings
make deny       # Check supply chain security
```

## Architecture

The codebase follows Clean Architecture with four layers:

```
cli/           → Command parsing (clap) and dispatch
application/   → Use cases orchestrating domain + infrastructure
domain/        → Core business logic, pure Rust, no I/O
infrastructure/→ External concerns: filesystem, parsing, rendering
```

### Layer Dependencies

```
cli → application → domain
         ↓
    infrastructure
```

### Key Patterns

**Use Cases** (`application/`): Each command has a use case struct that:
- Takes a `FileSystem` trait for testability
- Has an `Options` struct (builder pattern) for configuration
- Returns a `Result` struct with operation results

```rust
let use_case = GenerateUseCase::new(fs);
let options = GenerateOptions::new("docs/decisions").with_output("viewer.html");
let result = use_case.execute(&options)?;
```

**FileSystem Trait** (`infrastructure/fs.rs`): All file I/O goes through the `FileSystem` trait. Tests use `InMemoryFileSystem` (enabled via `#[cfg(any(test, feature = "testing"))]`).

**Domain Types** (`domain/`):
- `Adr` - Parsed ADR with frontmatter and content
- `Frontmatter` - YAML metadata (title, status, tags, etc.)
- `Status` - Enum: Proposed, Accepted, Deprecated, Superseded
- `Facets` - Search facets extracted from ADRs
- `Graph` - Relationship graph between ADRs
- `ValidationReport` - Validation results with issues

### Parser Pipeline

```
markdown file → frontmatter extraction → YAML parse → Adr struct
                (infrastructure/parser/frontmatter.rs)
```

### Renderer Pipeline

```
Vec<Adr> → Facets + Graph → Askama template → HTML
           (domain/)         (infrastructure/renderer/)
```

## Code Constraints

- **No panics in library code**: `unwrap`, `expect`, `panic!` are denied by clippy
- **No unsafe code**: `#![forbid(unsafe_code)]`
- **All errors use `Result`**: Custom `Error` enum in `error.rs` with `thiserror`
- **MSRV**: Rust 1.85 (2024 edition)

## Testing

- Unit tests: Inside each module with `#[cfg(test)]`
- Integration tests: `tests/integration_test.rs`
- Use `InMemoryFileSystem` for testing without touching disk
- Property tests with `proptest` for parser edge cases
