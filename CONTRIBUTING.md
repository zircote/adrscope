# Contributing to ADRScope

Thank you for your interest in contributing to ADRScope! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust**: Version 1.85 or later
- **Git**: For version control
- **Make**: Optional but recommended for build shortcuts

### Setting Up Your Development Environment

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/adrscope.git
   cd adrscope
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   make test
   # or
   cargo test
   ```

4. **Run linters**:
   ```bash
   make lint
   # or
   cargo clippy --all-targets --all-features -- -D warnings
   ```

## Development Workflow

### Branch Naming

Use descriptive branch names following this pattern:

- `feat/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test additions or modifications

Example: `feat/add-json-export` or `fix/parser-edge-case`

### Making Changes

1. **Create a new branch**:
   ```bash
   git checkout -b feat/my-new-feature
   ```

2. **Make your changes** following the [coding standards](#coding-standards)

3. **Run the full check suite**:
   ```bash
   make check
   ```

4. **Commit your changes** with clear, descriptive messages:
   ```bash
   git commit -m "feat: add JSON export functionality"
   ```

   We follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` - New features
   - `fix:` - Bug fixes
   - `docs:` - Documentation changes
   - `refactor:` - Code refactoring
   - `test:` - Test changes
   - `chore:` - Build/tooling changes

5. **Push your branch**:
   ```bash
   git push origin feat/my-new-feature
   ```

6. **Create a Pull Request** on GitHub

## Coding Standards

ADRScope follows strict coding standards enforced by Rust's compiler and tooling.

### Architecture Principles

ADRScope uses **Clean Architecture** with four layers:

```
cli → application → domain
         ↓
    infrastructure
```

**Key rules**:
- **No panics in library code**: Use `Result` types instead
- **No unsafe code**: `#![forbid(unsafe_code)]`
- **Pure domain logic**: The domain layer must have zero I/O
- **Trait abstractions**: All I/O goes through the `FileSystem` trait

See [Architecture Documentation](docs/architecture.md) for details.

### Code Style

We use `rustfmt` with project-specific configuration:

```bash
cargo fmt --all
```

Configuration is in `rustfmt.toml`. Key settings:
- Max line width: 100 characters
- Edition 2024 formatting
- Consistent import grouping

### Linting

We enforce strict linting with clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Enabled lint groups:
- `clippy::all` - All standard lints
- `clippy::pedantic` - Pedantic lints
- `clippy::nursery` - Experimental lints
- `missing_docs` - Require documentation for public items

Some lints are allowed for practical reasons (see `clippy.toml` and `src/lib.rs`).

### Documentation

All public items must have documentation comments:

```rust
/// Generates an HTML viewer for Architecture Decision Records.
///
/// # Arguments
///
/// * `adrs` - The ADRs to include in the viewer.
/// * `options` - Configuration options for generation.
///
/// # Returns
///
/// Returns `Ok(GenerateResult)` on success.
///
/// # Errors
///
/// Returns `Error::Io` if file operations fail.
///
/// # Examples
///
/// ```
/// use adrscope::application::{GenerateOptions, GenerateUseCase};
/// use adrscope::infrastructure::fs::RealFileSystem;
///
/// let fs = RealFileSystem::new();
/// let use_case = GenerateUseCase::new(fs);
/// let options = GenerateOptions::new("docs/decisions");
/// let result = use_case.execute(&options)?;
/// # Ok::<(), adrscope::Error>(())
/// ```
pub fn generate(adrs: &[Adr], options: &GenerateOptions) -> Result<GenerateResult> {
    // implementation
}
```

Follow these documentation guidelines:
- Use active voice
- Start with a verb ("Generates", "Parses", "Validates")
- Include examples for public APIs
- Document errors with `# Errors` section
- Document panics with `# Panics` section (though we avoid panics)

### Error Handling

All fallible operations must return `Result`:

```rust
// ✅ Good
pub fn parse_adr(content: &str) -> Result<Adr> {
    // ...
}

// ❌ Bad - panics on error
pub fn parse_adr(content: &str) -> Adr {
    // ...
}
```

Use the unified `Error` type from `src/error.rs`:

```rust
use crate::{Error, Result};

pub fn my_function() -> Result<String> {
    let content = std::fs::read_to_string("file.txt")
        .map_err(Error::Io)?;
    Ok(content)
}
```

Never use `unwrap()`, `expect()`, or `panic!()` in library code.

### Testing

Write comprehensive tests for all new code:

#### Unit Tests

Place unit tests in the same file with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let input = "---\ntitle: Test\n---\n";
        let result = parse_frontmatter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_frontmatter() {
        let input = "---\ninvalid yaml\n---\n";
        let result = parse_frontmatter(input);
        assert!(matches!(result, Err(Error::Parse(_))));
    }
}
```

#### Integration Tests

For end-to-end tests, add to `tests/integration_test.rs`:

```rust
#[test]
fn test_generate_command() {
    let fs = InMemoryFileSystem::new();
    fs.write(Path::new("adr.md"), SAMPLE_ADR).unwrap();
    
    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new(".").with_output("output.html");
    let result = use_case.execute(&options);
    
    assert!(result.is_ok());
}
```

#### Property-Based Tests

For parsers and complex logic, use `proptest`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_roundtrip(s in ".*") {
        let encoded = encode(&s);
        let decoded = decode(&encoded)?;
        prop_assert_eq!(s, decoded);
    }
}
```

### Test Coverage

We aim for high test coverage:
- All public functions should have tests
- Critical paths must have tests
- Edge cases should be covered

Run tests with:
```bash
make test
# or
cargo test --all-features
```

## Pull Request Process

### Before Submitting

Ensure your PR passes all checks:

```bash
make ci
```

This runs:
1. Format check (`cargo fmt --check`)
2. Clippy lints (`cargo clippy -D warnings`)
3. All tests (`cargo test`)
4. Documentation build (`cargo doc`)
5. Supply chain audit (`cargo deny check`)
6. MSRV verification

### PR Description

Include in your PR description:
- **What**: Brief description of changes
- **Why**: Motivation for the change
- **How**: Technical approach (if complex)
- **Testing**: How you tested the changes
- **Breaking changes**: Any API changes or migrations needed

Example:
```markdown
## What
Add JSON export format for ADR statistics

## Why
Users requested machine-readable statistics output for integration with other tools.

## How
- Added `--format json` flag to stats command
- Implemented JSON serialization using serde
- Updated documentation

## Testing
- Added unit tests for JSON serialization
- Added integration test for stats --format json
- Manually tested with real ADR corpus

## Breaking Changes
None - this is additive functionality
```

### Review Process

1. Maintainers will review your PR
2. Address feedback by pushing new commits
3. Once approved, a maintainer will merge your PR

## Types of Contributions

### Bug Fixes

1. Check if an issue already exists
2. If not, create an issue describing the bug
3. Reference the issue in your PR

### New Features

1. Create an issue to discuss the feature first
2. Wait for maintainer feedback
3. Implement the feature following architecture guidelines
4. Update documentation
5. Add tests

### Documentation

Documentation improvements are always welcome:
- Fix typos
- Improve clarity
- Add examples
- Expand explanations

### Performance Improvements

1. Include benchmarks demonstrating the improvement
2. Explain the optimization technique
3. Ensure tests still pass

## Dependency Management

### Adding Dependencies

Be conservative when adding dependencies:
- Prefer standard library when possible
- Check license compatibility (MIT, Apache 2.0, BSD)
- Verify with `cargo deny check`
- Justify the dependency in your PR description

### Updating Dependencies

- Run `cargo update` to update within semver ranges
- Test thoroughly after updates
- Check for security advisories

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.X.Y`
4. Push tag: `git push origin v0.X.Y`
5. CI will automatically publish to crates.io and GitHub releases

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/zircote/adrscope/discussions)
- **Bugs**: Open a [GitHub Issue](https://github.com/zircote/adrscope/issues)
- **Security**: Email zircote@gmail.com (do not open public issues)

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes
- Special thanks in README (for significant contributions)

## License

By contributing to ADRScope, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to ADRScope! 🎉
