# Contributing to ADRScope

Thank you for your interest in contributing to ADRScope! This document provides guidelines and workflows for contributing.

## Code of Conduct

Be respectful, inclusive, and collaborative. We're all here to build something useful together.

## Getting Started

### Prerequisites

- **Rust 1.85+** (2024 edition)
- **cargo-deny** - `cargo install cargo-deny`
- **Git** with [Conventional Commits](https://www.conventionalcommits.org/) knowledge

### Clone and Build

````bash
git clone https://github.com/zircote/adrscope.git
cd adrscope
make check
````

This runs the quick check pipeline: format, lint, and test.

## Development Workflow

### 1. Create a Branch

````bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
````

**Branch Naming:**

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 2. Make Changes

Follow the [code conventions](#code-conventions) below.

### 3. Run Checks

Before committing, run:

````bash
make check
````

This executes:
- `cargo fmt --check` - Code formatting
- `cargo clippy -- -D warnings` - Linting
- `cargo test` - All tests

For a full CI-style check:

````bash
make ci
````

This adds:
- `cargo doc` - Documentation generation
- `cargo deny check` - Supply chain security
- MSRV verification

### 4. Commit Changes

Use [Conventional Commits](https://www.conventionalcommits.org/):

````bash
git commit -m "feat: add faceted search for technologies"
git commit -m "fix: handle empty ADR frontmatter gracefully"
git commit -m "docs: update library API examples"
````

**Commit Types:**

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, missing semicolons, etc.) |
| `refactor` | Code refactoring without behavior change |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Build process, tooling, dependencies |

### 5. Push and Open PR

````bash
git push origin feature/your-feature-name
````

Open a pull request on GitHub with:

- **Clear title** following conventional commits format
- **Description** explaining what and why
- **Testing notes** showing how you verified the changes
- **Breaking changes** if applicable

## Code Conventions

### Architecture

ADRScope follows **Clean Architecture** with four layers:

````
src/
├── cli/              # Command-line interface
├── application/      # Use cases (orchestration)
├── domain/           # Core business logic (pure, no I/O)
└── infrastructure/   # External concerns (fs, parsing, rendering)
````

**Dependency Rules:**

- `cli` → `application` → `domain`
- `application` → `infrastructure` (for I/O)
- `domain` NEVER depends on `infrastructure`

### Error Handling

- **No panics** - Use `Result` types
- **No `unwrap()`** in library code (tests are OK)
- **Use `thiserror`** for custom errors

````rust
// Good
pub fn parse_value(input: &str) -> Result<i64> {
    input.parse().map_err(|e| Error::Parse(e))
}

// Bad
pub fn parse_value(input: &str) -> i64 {
    input.parse().unwrap()  // ❌ Never in library code
}
````

### Safety

- **`#![forbid(unsafe_code)]`** - No unsafe code allowed
- Use safe Rust patterns exclusively

### Documentation

Document public APIs with examples:

````rust
/// Generates an HTML viewer from ADRs.
///
/// # Arguments
///
/// * `options` - Configuration for generation
///
/// # Returns
///
/// A result containing the number of ADRs processed.
///
/// # Errors
///
/// Returns an error if:
/// - No ADR files are found
/// - File reading fails
/// - HTML rendering fails
///
/// # Examples
///
/// ````no_run
/// use adrscope::application::{GenerateOptions, GenerateUseCase};
/// use adrscope::infrastructure::fs::RealFileSystem;
///
/// let fs = RealFileSystem::new();
/// let use_case = GenerateUseCase::new(fs);
/// let options = GenerateOptions::new("docs/decisions");
///
/// let result = use_case.execute(&options)?;
/// # Ok::<(), adrscope::Error>(())
/// ````
pub fn execute(&self, options: &GenerateOptions) -> Result<GenerateResult> {
    // implementation
}
````

### Testing

Write comprehensive tests:

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_frontmatter() {
        let input = "---\ntitle: Test\nstatus: accepted\n---\n\n## Context\n\nTest";
        let result = parse(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().frontmatter.title, "Test");
    }

    #[test]
    fn test_parse_missing_title_returns_error() {
        let input = "---\nstatus: accepted\n---\n\n## Context\n\nTest";
        let result = parse(input);
        assert!(matches!(result, Err(Error::Parse(_))));
    }
}
````

Use `InMemoryFileSystem` for testing:

````rust
#[test]
fn test_generate_use_case() {
    let mut fs = InMemoryFileSystem::new();
    fs.add_file("docs/adr-001.md", "---\ntitle: Test\nstatus: accepted\n---\n\n## Context\n\nTest");
    
    let use_case = GenerateUseCase::new(fs);
    let result = use_case.execute(&GenerateOptions::default()).unwrap();
    
    assert_eq!(result.adr_count, 1);
}
````

### Linting

We use clippy with pedantic and nursery lints:

````bash
cargo clippy -- -D warnings
````

Some lints are allowed for practical reasons (see `src/lib.rs` for the full list).

### Formatting

Use rustfmt with our configuration:

````bash
cargo fmt
````

Configuration is in `rustfmt.toml`.

## Testing

### Run All Tests

````bash
make test
````

Or with cargo directly:

````bash
cargo test --verbose
````

### Run Specific Test

````bash
cargo test test_name
````

### Run with Output

````bash
cargo test -- --nocapture
````

### Integration Tests

Integration tests are in `tests/integration_test.rs`. They test the full pipeline with real file fixtures.

## Documentation

### Build Documentation

````bash
make doc
````

Or:

````bash
cargo doc --open
````

### Update User Guides

User-facing documentation is in `docs/`:

- `getting-started.md` - Tutorial
- `user-guide.md` - Command reference
- `configuration.md` - Configuration options
- `library-api.md` - Library usage guide

When adding features, update relevant documentation.

### ADRs

Architectural decisions are documented in `docs/decisions/`. When making significant architectural changes, create a new ADR:

````bash
cp docs/decisions/adr-0001-template.md docs/decisions/adr-NNNN-your-decision.md
````

Follow the [structured-MADR](https://github.com/zircote/structured-madr) format.

## Pull Request Guidelines

### PR Checklist

- [ ] Code follows clean architecture principles
- [ ] All tests pass (`make test`)
- [ ] Linting passes (`make lint`)
- [ ] Code is formatted (`make fmt`)
- [ ] Documentation updated if needed
- [ ] ADR created for architectural changes
- [ ] Conventional commit messages used
- [ ] PR description explains what and why

### Review Process

1. **Automated checks** - CI runs on all PRs
2. **Code review** - Maintainer reviews for quality and design
3. **Feedback** - Address review comments
4. **Approval** - Maintainer approves changes
5. **Merge** - Squash and merge to main

### CI Checks

GitHub Actions runs:

- ✅ Format check (`cargo fmt --check`)
- ✅ Clippy (`cargo clippy -- -D warnings`)
- ✅ Tests (`cargo test`)
- ✅ Documentation (`cargo doc`)
- ✅ Supply chain security (`cargo deny check`)
- ✅ MSRV verification (Rust 1.85)

All checks must pass before merging.

## Release Process

Releases are handled by maintainers:

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml`
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions builds and publishes to crates.io

## Makefile Commands

Quick reference:

````bash
make build          # Build debug binary
make release        # Build optimized binary
make test           # Run all tests
make lint           # Run clippy
make fmt            # Format code
make check          # Quick check (fmt + lint + test)
make ci             # Full CI pipeline
make doc            # Generate documentation
make install        # Install to ~/.cargo/bin
make clean          # Clean build artifacts
````

## Getting Help

- **Issues** - [GitHub Issues](https://github.com/zircote/adrscope/issues)
- **Discussions** - [GitHub Discussions](https://github.com/zircote/adrscope/discussions)
- **Chat** - Check README for community links

## Recognition

Contributors are recognized in:

- Git commit history
- Release notes in `CHANGELOG.md`
- GitHub contributors graph

Thank you for contributing! 🎉
