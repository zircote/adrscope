# Developer Guide

Guide for contributing to ADRScope and understanding its architecture.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Code Organization](#code-organization)
- [Testing Strategy](#testing-strategy)
- [Adding Features](#adding-features)
- [Code Quality](#code-quality)
- [Release Process](#release-process)

## Architecture Overview

ADRScope follows **Clean Architecture** principles with clear layer separation:

````text
┌─────────────────────────────────────────────────────┐
│                    CLI Layer                        │
│  (Command parsing, dispatch, output formatting)     │
└─────────────────────┬───────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────┐
│                Application Layer                    │
│  (Use cases: Generate, Validate, Stats, Wiki)      │
└──────────┬────────────────────────────┬─────────────┘
           │                            │
           ▼                            ▼
┌──────────────────────┐    ┌──────────────────────────┐
│   Domain Layer       │    │  Infrastructure Layer    │
│  (Core entities,     │    │  (Filesystem, Parsing,   │
│   business logic)    │    │   Rendering)             │
└──────────────────────┘    └──────────────────────────┘
````

### Layer Responsibilities

**CLI Layer** (`src/cli/`)
- Parse command-line arguments with `clap`
- Dispatch to appropriate use cases
- Format output for terminal display
- Handle exit codes

**Application Layer** (`src/application/`)
- Orchestrate use cases (Generate, Validate, Stats, Wiki)
- Coordinate domain logic and infrastructure services
- Return structured results
- No I/O or external dependencies

**Domain Layer** (`src/domain/`)
- Pure business logic
- Core entities: ADR, Frontmatter, Status, Graph, Facets
- Validation rules
- Statistics computation
- No I/O, completely testable in isolation

**Infrastructure Layer** (`src/infrastructure/`)
- External integrations: filesystem, parsing, rendering
- Trait implementations for dependency injection
- Parser implementation (frontmatter, markdown)
- HTML and Wiki rendering with Askama templates

### Design Patterns

**Dependency Injection**
- Use cases accept `FileSystem` trait for testability
- Enables both `RealFileSystem` and `InMemoryFileSystem`

**Builder Pattern**
- All `*Options` structs use builder pattern
- Fluent API with `with_*` methods

**Strategy Pattern**
- `ValidationRule` trait for pluggable validation
- `AdrParser` trait for custom parsing

**Result Pattern**
- All fallible operations return `Result<T, Error>`
- No panics in library code (enforced by clippy)

## Development Setup

### Prerequisites

- **Rust 1.85+** (2024 edition)
- `cargo-deny` for supply chain security
- `cargo-msrv` for MSRV verification (optional)

### Clone and Build

````bash
git clone https://github.com/zircote/adrscope.git
cd adrscope
make build
````

### Run Tests

````bash
# All tests
make test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Watch mode (requires cargo-watch)
cargo watch -x test
````

### Development Workflow

````bash
# Quick feedback loop (format + lint + test)
make check

# Full CI pipeline locally
make ci

# Run specific commands
cargo run -- generate -i docs/decisions
cargo run -- validate --strict
cargo run -- stats
````

## Project Structure

````text
adrscope/
├── src/
│   ├── application/        # Use cases
│   │   ├── generate.rs     # HTML viewer generation
│   │   ├── validate.rs     # ADR validation
│   │   ├── stats.rs        # Statistics computation
│   │   └── wiki.rs         # Wiki page generation
│   ├── cli/                # Command-line interface
│   │   ├── args.rs         # Argument parsing
│   │   └── handlers.rs     # Command handlers
│   ├── domain/             # Core business logic
│   │   ├── adr.rs          # ADR entity
│   │   ├── frontmatter.rs  # YAML metadata
│   │   ├── status.rs       # Status enum
│   │   ├── graph.rs        # Relationship graph
│   │   ├── facets.rs       # Search facets
│   │   ├── stats.rs        # Statistics types
│   │   └── validation.rs   # Validation framework
│   ├── infrastructure/     # External integrations
│   │   ├── fs.rs           # Filesystem abstraction
│   │   ├── parser/         # ADR parsing
│   │   │   ├── frontmatter.rs
│   │   │   ├── markdown.rs
│   │   │   └── mod.rs
│   │   └── renderer/       # HTML/Wiki generation
│   │       ├── html.rs
│   │       ├── wiki.rs
│   │       └── mod.rs
│   ├── error.rs            # Error types
│   ├── lib.rs              # Library entry point
│   └── main.rs             # Binary entry point
├── templates/              # Askama templates
│   ├── viewer.html         # HTML viewer template
│   ├── styles.css          # Embedded CSS
│   └── app.js              # Embedded JavaScript
├── tests/                  # Integration tests
│   └── integration_test.rs
├── docs/                   # Documentation
│   ├── getting-started.md
│   ├── user-guide.md
│   ├── configuration.md
│   ├── api-reference.md
│   ├── developer-guide.md  # This file
│   └── decisions/          # ADRs documenting ADRScope
├── Cargo.toml              # Dependencies and metadata
├── Makefile                # Build automation
├── clippy.toml             # Clippy configuration
├── rustfmt.toml            # Rustfmt configuration
└── deny.toml               # Cargo-deny configuration
````

## Code Organization

### Module Guidelines

**Public vs. Private**
- Mark types as `pub` only if they're part of the public API
- Keep implementation details private
- Use `pub(crate)` for internal sharing across modules

**Module Structure**
````rust
// Public types at the top
pub struct MyType { ... }

// Public functions
pub fn create() -> MyType { ... }

// Private helpers at the bottom
fn internal_helper() { ... }

// Tests at the end
#[cfg(test)]
mod tests { ... }
````

### Naming Conventions

**Types**
- Structs: `PascalCase` (e.g., `GenerateUseCase`)
- Enums: `PascalCase` (e.g., `Status`)
- Traits: `PascalCase` (e.g., `FileSystem`)

**Functions**
- snake_case (e.g., `parse_frontmatter`)
- Builders: `with_*` prefix (e.g., `with_output`)
- Converters: `from_*`, `to_*`, `as_*`
- Predicates: `is_*`, `has_*`

**Variables**
- snake_case (e.g., `adr_count`)
- Avoid single-letter names except in closures

### Error Handling

**Always use `Result`**
````rust
// Good
pub fn parse(content: &str) -> Result<Adr> {
    // ...
}

// Bad - panics are forbidden
pub fn parse(content: &str) -> Adr {
    let data = content.parse().unwrap(); // ❌ Never do this
    // ...
}
````

**Custom Errors**
````rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read file {path}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("missing required field: {field}")]
    MissingField { field: String },
}
````

### Documentation

**Public Items**
````rust
/// Generates an HTML viewer for ADRs.
///
/// # Arguments
///
/// * `options` - Configuration for generation
///
/// # Returns
///
/// A `GenerateResult` containing the ADR count and output path.
///
/// # Errors
///
/// Returns an error if:
/// - Input directory doesn't exist
/// - No ADR files found matching pattern
/// - Template rendering fails
/// - File write fails
///
/// # Examples
///
/// ```
/// use adrscope::application::{GenerateUseCase, GenerateOptions};
/// use adrscope::infrastructure::fs::RealFileSystem;
///
/// let fs = RealFileSystem::new();
/// let use_case = GenerateUseCase::new(fs);
/// let options = GenerateOptions::new("docs/decisions");
/// let result = use_case.execute(&options)?;
/// # Ok::<(), adrscope::Error>(())
/// ```
pub fn execute(&self, options: &GenerateOptions) -> Result<GenerateResult> {
    // ...
}
````

## Testing Strategy

### Unit Tests

Place unit tests in the same file as the code:

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status() {
        let status: Status = "accepted".parse().unwrap();
        assert_eq!(status, Status::Accepted);
    }

    #[test]
    fn test_unknown_status_defaults_to_proposed() {
        let status: Status = "unknown".parse().unwrap();
        assert_eq!(status, Status::Proposed);
    }
}
````

### Integration Tests

Use `tests/integration_test.rs` for end-to-end testing:

````rust
#[test]
fn test_generate_complete_workflow() {
    let mut fs = InMemoryFileSystem::new();
    fs.add_file(
        "adr-0001.md",
        "---\ntitle: Test\nstatus: accepted\n---\n# Test",
    );

    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new(".").with_output("viewer.html");

    let result = use_case.execute(&options).unwrap();
    assert_eq!(result.adr_count, 1);
}
````

### Property-Based Testing

Use `proptest` for complex logic:

````rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn status_round_trips(s in "[a-z]+") {
        let status: Status = s.parse()?;
        let serialized = status.to_string();
        let deserialized: Status = serialized.parse()?;
        assert_eq!(status, deserialized);
    }
}
````

### Test Coverage

Run coverage locally with `cargo-tarpaulin`:

````bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
open tarpaulin-report.html
````

## Adding Features

### Adding a New Use Case

1. **Create use case module** (`src/application/my_feature.rs`)

````rust
use crate::domain::Adr;
use crate::infrastructure::FileSystem;
use crate::Result;

pub struct MyFeatureUseCase<F: FileSystem> {
    fs: F,
}

impl<F: FileSystem> MyFeatureUseCase<F> {
    pub fn new(fs: F) -> Self {
        Self { fs }
    }

    pub fn execute(&self, options: &MyFeatureOptions) -> Result<MyFeatureResult> {
        // Implementation
        Ok(MyFeatureResult { /* ... */ })
    }
}

pub struct MyFeatureOptions {
    input: String,
}

impl MyFeatureOptions {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

pub struct MyFeatureResult {
    // Result data
}
````

2. **Add to application module** (`src/application/mod.rs`)

````rust
mod my_feature;
pub use my_feature::{MyFeatureOptions, MyFeatureResult, MyFeatureUseCase};
````

3. **Add CLI command** (`src/cli/args.rs`)

````rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands
    MyFeature(MyFeatureArgs),
}

#[derive(Args)]
pub struct MyFeatureArgs {
    #[arg(short, long, default_value = "docs/decisions")]
    input: String,
}
````

4. **Add handler** (`src/cli/handlers.rs`)

````rust
pub fn handle_my_feature(args: &MyFeatureArgs) -> Result<()> {
    let fs = RealFileSystem::new();
    let use_case = MyFeatureUseCase::new(fs);
    let options = MyFeatureOptions::new(&args.input);
    let result = use_case.execute(&options)?;
    // Output formatting
    Ok(())
}
````

### Adding a Validation Rule

1. **Create rule struct**

````rust
use crate::domain::{Adr, ValidationIssue, ValidationRule, Severity};

pub struct MyCustomRule;

impl ValidationRule for MyCustomRule {
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Validation logic
        if /* condition */ {
            issues.push(ValidationIssue {
                file: adr.id.to_string(),
                severity: Severity::Warning,
                message: "Issue description".to_string(),
                line: None,
            });
        }

        issues
    }
}
````

2. **Add to default rules** (if appropriate)

````rust
pub fn default_rules() -> Vec<Box<dyn ValidationRule>> {
    vec![
        Box::new(RequiredFieldsRule),
        Box::new(RecommendedFieldsRule),
        Box::new(MyCustomRule), // Add here
    ]
}
````

### Adding a Domain Type

1. **Create type in domain layer**

````rust
// src/domain/my_type.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyType {
    pub field: String,
}

impl MyType {
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = MyType::new("test");
        assert_eq!(t.field, "test");
    }
}
````

2. **Export from domain module**

````rust
// src/domain/mod.rs
mod my_type;
pub use my_type::MyType;
````

## Code Quality

### Linting

Run clippy with pedantic lints:

````bash
make lint

# Or manually
cargo clippy --all-targets --all-features -- -D warnings
````

**Clippy Configuration** (`clippy.toml`)
- Pedantic and nursery lints enabled
- Some lints allowed for practical reasons (see `src/lib.rs`)

### Formatting

Format code with rustfmt:

````bash
make fmt

# Check without modifying
cargo fmt -- --check
````

**Formatting Rules** (`rustfmt.toml`)
- Max width: 100 characters
- Edition: 2024
- Use field init shorthand
- Imports granularity: Crate

### Supply Chain Security

Check dependencies with cargo-deny:

````bash
make deny

# Or manually
cargo deny check
````

**Deny Configuration** (`deny.toml`)
- Advisory checks for security vulnerabilities
- License compliance (MIT/Apache-2.0)
- Banned crates enforcement
- Duplicate dependency detection

### MSRV Verification

Verify Minimum Supported Rust Version:

````bash
# Using cargo-msrv
cargo install cargo-msrv
cargo msrv verify

# Manual verification
rustup install 1.85
cargo +1.85 build
cargo +1.85 test
````

## Release Process

### Version Bumping

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit changes

````bash
# Example for 0.4.0 release
vim Cargo.toml  # Update version
vim CHANGELOG.md  # Add release notes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: prepare 0.4.0 release"
````

### Tagging

````bash
git tag -a v0.4.0 -m "Release 0.4.0"
git push origin v0.4.0
````

### GitHub Release

The GitHub Actions workflow (`.github/workflows/release.yml`) automatically:
1. Builds binaries for all platforms
2. Publishes to crates.io
3. Creates GitHub release with assets
4. Updates Homebrew tap

### Manual Publishing

If needed, publish manually:

````bash
# Publish to crates.io
cargo publish

# Dry run first
cargo publish --dry-run
````

## Common Tasks

### Update Dependencies

````bash
# Check for outdated dependencies
cargo outdated

# Update within semver constraints
cargo update

# Update to latest major versions
cargo upgrade
````

### Benchmark Performance

````bash
# Run benchmarks (requires criterion)
cargo bench

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph -- generate -i docs/decisions
````

### Generate Documentation

````bash
# Generate and open docs
cargo doc --open

# With private items
cargo doc --document-private-items --open
````

## Troubleshooting

### Compilation Errors

**Problem:** `error: failed to compile adrscope`

**Solution:**
1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check MSRV: Ensure Rust 1.85+

### Test Failures

**Problem:** Tests failing locally but passing in CI

**Solution:**
1. Ensure clean state: `cargo clean`
2. Check for filesystem dependencies
3. Verify test isolation (no shared state)

### Clippy Warnings

**Problem:** New clippy warnings after Rust update

**Solution:**
1. Review warnings: `cargo clippy`
2. Fix or allow specific lints
3. Update `.clippy.toml` if needed

## Contributing Checklist

Before submitting a PR:

- [ ] Run `make check` successfully
- [ ] Add tests for new functionality
- [ ] Update documentation
- [ ] Run `make ci` for full validation
- [ ] Follow conventional commit format
- [ ] Add CHANGELOG entry (if user-facing)
- [ ] Verify MSRV compatibility

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clean Architecture in Rust](https://www.possiblerust.com/pattern/clean-architecture-in-rust)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

## Questions?

- Open an issue on GitHub
- Check existing ADRs in `docs/decisions/`
- Review the API documentation
