---
title: Use thiserror for Unified Error Types with Rich Context
description: Decision to use thiserror crate for defining a single error enum with contextual information and source error chaining
type: adr
category: api, error-handling
tags:
  - error-handling
  - api-design
  - thiserror
  - rust-patterns
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
  - thiserror
audience:
  - developers
  - library-consumers
related:
  - adr-0004-forbid-unsafe-code-and-panics.md
---

## Context

Errors in ADRScope can originate from multiple sources:

- **I/O operations**: File reading, directory traversal, path resolution
- **YAML parsing**: Frontmatter extraction, metadata validation
- **Template rendering**: Variable substitution, format errors

The application needs a robust error handling strategy that provides:

- A single error type for public API consistency
- Contextual information (file paths, field names) for debugging
- Source error chaining for root cause analysis
- Compatibility with Rust's `?` operator for ergonomic propagation

Without a unified approach, consumers face multiple error types, inconsistent error messages, and difficulty tracing errors to their source.

## Decision

We will define a single `Error` enum using the `thiserror` crate with context-rich variants.

### Implementation Approach

```rust
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid YAML frontmatter in {path}: {details}")]
    FrontmatterParse {
        path: PathBuf,
        details: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("missing required field '{field}' in {path}")]
    MissingField {
        path: PathBuf,
        field: String,
    },

    #[error("template rendering failed: {template}")]
    TemplateRender {
        template: String,
        #[source]
        source: tera::Error,
    },

    #[error("invalid ADR format: {0}")]
    InvalidFormat(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

### Key Design Principles

1. **Single public error type**: All public functions return `Result<T, Error>`
2. **Rich context**: Each variant includes relevant paths, field names, or identifiers
3. **Source chaining**: Use `#[source]` attribute to preserve original errors
4. **Display messages**: Human-readable messages via `#[error(...)]` attribute
5. **Debug derivation**: Full debug output for development and logging

### Usage Pattern

```rust
pub fn parse_adr(path: &Path) -> Result<Adr> {
    let content = std::fs::read_to_string(path)
        .map_err(|source| Error::FileRead {
            path: path.to_path_buf(),
            source,
        })?;

    let frontmatter = extract_frontmatter(&content)
        .map_err(|source| Error::FrontmatterParse {
            path: path.to_path_buf(),
            details: "could not parse YAML block".to_string(),
            source,
        })?;

    // ...
}
```

## Consequences

### Positive

- **Single error type simplifies public API**: Consumers only need to handle one `Error` type, reducing cognitive load and simplifying match expressions
- **Rich context aids debugging**: File paths and field names in error messages enable rapid issue identification
- **`#[source]` enables error chain inspection**: Tools like `anyhow` and logging frameworks can traverse the full error chain
- **Compile-time exhaustiveness**: Adding new variants forces handling at all match sites
- **Zero-cost abstractions**: `thiserror` generates efficient code with no runtime overhead

### Negative

- **Enum grows with each new error case**: As the crate evolves, the error enum may become large; consider nested error types if this becomes unwieldy
- **Context duplication**: Similar contexts (e.g., file paths) appear in multiple variants, requiring consistent handling

### Trade-offs

- **Comprehensive error context vs. enum size**: Rich context improves debugging but increases the size of the error type; this is an acceptable trade-off for a library focused on developer experience
- **Structured variants vs. string messages**: Structured variants enable programmatic error handling but require more upfront design than simple string errors
