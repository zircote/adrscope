# Architecture

This document describes the high-level architecture of ADRScope.

## Overview

ADRScope follows **Clean Architecture** principles with four distinct layers:

````
┌─────────────────────────────────────────┐
│              CLI Layer                  │
│   (Command parsing & dispatch)          │
└──────────────┬──────────────────────────┘
               │
               v
┌─────────────────────────────────────────┐
│         Application Layer               │
│   (Use cases & orchestration)           │
└──────┬──────────────────────────────────┘
       │                 │
       v                 v
┌──────────────┐  ┌─────────────────────┐
│   Domain     │  │  Infrastructure     │
│   (Business  │  │  (I/O, Parsing,     │
│    Logic)    │  │   Rendering)        │
└──────────────┘  └─────────────────────┘
````

### Design Principles

1. **Dependency Inversion**: High-level modules don't depend on low-level modules. Both depend on abstractions (traits).
2. **Pure Domain Logic**: The domain layer has zero I/O dependencies and is pure Rust.
3. **Testability**: All file I/O goes through the `FileSystem` trait, enabling in-memory testing.
4. **Error Handling**: All fallible operations return `Result` types; no panics in library code.

## Layer Details

### CLI Layer (`src/cli/`)

**Purpose**: Parse command-line arguments and dispatch to appropriate use cases.

**Key Components**:
- `args.rs` - Command-line argument definitions using `clap`
- `handlers.rs` - Dispatch logic connecting CLI commands to use cases

**Dependencies**: Application layer

**Example**:
```rust
pub fn run() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Generate(opts) => handlers::handle_generate(opts),
        Command::Validate(opts) => handlers::handle_validate(opts),
        // ...
    }
}
```

### Application Layer (`src/application/`)

**Purpose**: Orchestrate business logic and coordinate between domain and infrastructure.

**Key Components**:
- `generate.rs` - HTML viewer generation use case
- `validate.rs` - ADR validation use case
- `stats.rs` - Statistics computation use case
- `wiki.rs` - GitHub Wiki generation use case

**Pattern**: Each use case follows this structure:

```rust
pub struct GenerateUseCase<F: FileSystem> {
    fs: F,
}

pub struct GenerateOptions {
    input_dir: PathBuf,
    output: PathBuf,
    // ...
}

impl GenerateOptions {
    pub fn new(input: impl Into<PathBuf>) -> Self { /* ... */ }
    pub fn with_output(mut self, output: impl Into<PathBuf>) -> Self { /* ... */ }
}

pub struct GenerateResult {
    pub adr_count: usize,
    pub output_path: PathBuf,
}

impl<F: FileSystem> GenerateUseCase<F> {
    pub fn new(fs: F) -> Self { Self { fs } }
    
    pub fn execute(&self, options: &GenerateOptions) -> Result<GenerateResult> {
        // 1. Find ADR files
        // 2. Parse ADRs (using infrastructure/parser)
        // 3. Build facets and graphs (using domain)
        // 4. Render HTML (using infrastructure/renderer)
        // 5. Write output
    }
}
```

**Dependencies**: Domain layer, Infrastructure layer (through traits)

### Domain Layer (`src/domain/`)

**Purpose**: Core business logic with zero external dependencies.

**Key Components**:

| Module | Purpose |
|--------|---------|
| `adr.rs` | Core ADR data structure |
| `frontmatter.rs` | YAML frontmatter metadata |
| `status.rs` | ADR status enum (Proposed, Accepted, etc.) |
| `facets.rs` | Search facet extraction and filtering |
| `graph.rs` | Relationship graph between ADRs |
| `stats.rs` | Statistics computation |
| `validation.rs` | Validation rules and reporting |

**Characteristics**:
- Pure functions wherever possible
- No I/O operations
- No `unwrap()` or `panic!()` in library code
- Comprehensive unit tests

**Example**:
```rust
// Pure function - no I/O, testable
pub fn extract_facets(adrs: &[Adr]) -> Facets {
    let mut facets = Facets::new();
    for adr in adrs {
        facets.add_status(adr.frontmatter.status);
        facets.add_category(adr.frontmatter.category.clone());
        // ...
    }
    facets
}
```

**Dependencies**: None (pure Rust standard library)

### Infrastructure Layer (`src/infrastructure/`)

**Purpose**: Handle external concerns (file I/O, parsing, rendering).

**Key Components**:

#### FileSystem Abstraction (`fs.rs`)

Trait-based abstraction for all file operations:

```rust
pub trait FileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, contents: &str) -> Result<()>;
    fn glob(&self, pattern: &str) -> Result<Vec<PathBuf>>;
}

// Production implementation
pub struct RealFileSystem;

// Test implementation (enabled via #[cfg(any(test, feature = "testing"))])
#[cfg(any(test, feature = "testing"))]
pub struct InMemoryFileSystem {
    files: HashMap<PathBuf, String>,
}
```

#### Parser (`parser/`)

- `frontmatter.rs` - Extract and parse YAML frontmatter from Markdown
- `markdown.rs` - Parse Markdown content
- `mod.rs` - Orchestrate parsing pipeline

**Pipeline**:
```
Markdown file → Extract frontmatter → Parse YAML → Parse content → Adr struct
```

#### Renderer (`renderer/`)

- `html.rs` - Generate self-contained HTML viewer using Askama templates
- `wiki.rs` - Generate GitHub Wiki pages in Markdown

**Dependencies**: Domain layer

## Data Flow

### Generate Command Flow

```
1. CLI parses arguments
   ↓
2. Application use case initialized
   ↓
3. Find ADR files (via FileSystem trait)
   ↓
4. Parse each file (infrastructure/parser)
   ↓
5. Extract facets (domain/facets)
   ↓
6. Build graph (domain/graph)
   ↓
7. Render template (infrastructure/renderer)
   ↓
8. Write output (via FileSystem trait)
```

### Validate Command Flow

```
1. CLI parses arguments
   ↓
2. Application use case initialized
   ↓
3. Find ADR files (via FileSystem trait)
   ↓
4. Parse each file (infrastructure/parser)
   ↓
5. Run validation rules (domain/validation)
   ↓
6. Generate report (domain/validation)
   ↓
7. Display results (CLI)
```

## Testing Strategy

### Unit Tests

- Located in each module with `#[cfg(test)]`
- Use `InMemoryFileSystem` for testing without disk I/O
- Property-based tests with `proptest` for parsers

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_facets() {
        let adrs = vec![
            Adr::new(/* ... */),
            Adr::new(/* ... */),
        ];
        let facets = extract_facets(&adrs);
        assert_eq!(facets.statuses.len(), 2);
    }
}
```

### Integration Tests

- Located in `tests/integration_test.rs`
- Test full command flows end-to-end
- Use `InMemoryFileSystem` for reproducibility

## Error Handling

All errors flow through a unified `Error` type defined in `src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Validation failed: {0}")]
    Validation(String),
    
    // ...
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Constraints**:
- No `unwrap()` or `expect()` in library code (enforced by clippy)
- No `panic!()` in library code
- All public functions return `Result<T>` for fallible operations

## Code Constraints

The codebase enforces strict quality standards:

1. **No unsafe code**: `#![forbid(unsafe_code)]`
2. **Comprehensive linting**: Clippy pedantic + nursery lints enabled
3. **Documentation**: All public items must have doc comments
4. **MSRV**: Rust 1.85 (2024 edition)

See `clippy.toml` and `rustfmt.toml` for complete linting and formatting rules.

## Build System

### Makefile Targets

```bash
make check      # Quick check: fmt + lint + test
make ci         # Full CI: fmt + lint + test + doc + deny + msrv
make test       # Run tests with verbose output
make lint       # Run clippy with -D warnings
make deny       # Check supply chain security
```

### Supply Chain Security

ADRScope uses `cargo-deny` to ensure supply chain security:

- License compliance checking
- Vulnerability scanning
- Duplicate dependency detection
- Source verification

See `deny.toml` for configuration.

## Extending ADRScope

### Adding a New Command

1. Define command arguments in `src/cli/args.rs`
2. Create use case in `src/application/`
3. Add handler in `src/cli/handlers.rs`
4. Add integration tests in `tests/integration_test.rs`

### Adding a New Validation Rule

1. Implement the `ValidationRule` trait in `src/domain/validation.rs`
2. Add the rule to `ValidationRuleSet`
3. Add tests for the new rule

Example:
```rust
pub struct MyCustomRule;

impl ValidationRule for MyCustomRule {
    fn name(&self) -> &'static str {
        "my-custom-rule"
    }
    
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        // validation logic
    }
}
```

## Performance Considerations

- **Compile-time templates**: Askama templates are compiled at build time for zero runtime overhead
- **Glob pattern matching**: Uses efficient globbing for file discovery
- **Minimal allocations**: Uses iterators and borrows where possible
- **Single-pass parsing**: Parse files once, reuse parsed data

## Security

- **No unsafe code**: All code is safe Rust
- **Input validation**: All user inputs are validated
- **Sanitized output**: HTML output is properly escaped
- **Supply chain auditing**: All dependencies are audited with `cargo-deny`

## Further Reading

- [Getting Started Guide](getting-started.md) - First steps with ADRScope
- [User Guide](user-guide.md) - Complete command reference
- [Configuration Reference](configuration.md) - All configuration options
- [API Documentation](https://docs.rs/adrscope) - Full API reference
- [ADR Index](decisions/README.md) - Architecture decisions for this project
