---
title: Use Trait-Based Filesystem Abstraction for Testability
description: Decision to abstract filesystem operations behind a trait to enable fast, deterministic unit testing without I/O
type: adr
category: architecture, testing
tags:
  - testing
  - abstraction
  - dependency-injection
  - traits
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
  - traits
audience:
  - developers
  - architects
related:
  - adr-0002-clean-architecture-layers.md
---

## Context

The application needs to read ADR files from the filesystem and write HTML output. Testing code that performs filesystem operations presents several challenges:

- **Actual file I/O**: Tests become slow and potentially flaky due to disk access, permissions, and timing issues
- **Mocking libraries**: Adding external mocking dependencies increases complexity and maintenance burden
- **Test fixtures on disk**: Requires consistent filesystem state across environments, making tests environment-dependent and harder to maintain in CI/CD pipelines

These challenges conflict with our goals of:

- Fast test execution for rapid feedback during development
- Deterministic tests that produce consistent results regardless of environment
- Isolated unit tests that do not require external resources
- Easy test setup without complex fixture management

## Decision

We will define a `FileSystem` trait that abstracts all filesystem operations, with two implementations:

### The FileSystem Trait

```rust
pub trait FileSystem: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String, Error>;
    fn write(&self, path: &Path, contents: &str) -> Result<(), Error>;
    fn glob(&self, pattern: &str) -> Result<Vec<PathBuf>, Error>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<(), Error>;
}
```

### Production Implementation

`RealFileSystem` wraps the standard library filesystem operations for production use:

```rust
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String, Error> {
        std::fs::read_to_string(path).map_err(Error::from)
    }
    // ... other methods delegate to std::fs
}
```

### Test Implementation

`InMemoryFileSystem` (available under `test_support` feature or `#[cfg(test)]`) stores files in a `HashMap` for unit testing:

```rust
pub struct InMemoryFileSystem {
    files: RwLock<HashMap<PathBuf, String>>,
}

impl InMemoryFileSystem {
    pub fn new() -> Self { ... }
    pub fn add_file(&self, path: impl AsRef<Path>, contents: &str) { ... }
}
```

### Usage Pattern

Use cases and services receive a `Box<dyn FileSystem>` or generic `F: FileSystem`:

```rust
pub struct LoadAdrUseCase<F: FileSystem> {
    fs: F,
}

impl<F: FileSystem> LoadAdrUseCase<F> {
    pub fn execute(&self, path: &Path) -> Result<Adr, Error> {
        let content = self.fs.read_to_string(path)?;
        // Parse and return ADR
    }
}
```

## Consequences

### Positive

- **Fast unit tests**: In-memory operations are orders of magnitude faster than disk I/O, enabling rapid test cycles
- **Deterministic results**: Tests produce identical results regardless of filesystem state, disk speed, or environment
- **Isolated testing**: Use cases can be fully tested without touching the actual filesystem
- **Easy test setup**: Test files are created programmatically without managing fixture directories
- **Parallel test execution**: No shared filesystem state means tests can safely run in parallel
- **CI/CD friendly**: No special filesystem permissions or pre-existing directories required

### Negative

- **Abstraction overhead**: Additional trait and implementation code to write and maintain
- **InMemoryFileSystem maintenance**: Must keep the in-memory implementation in sync with the trait as it evolves
- **Slight runtime cost**: Dynamic dispatch when using `Box<dyn FileSystem>` (negligible for I/O-bound operations)
- **Learning curve**: Contributors must understand the abstraction pattern

### Trade-offs

- **Test speed and isolation vs. implementation complexity**: We accept additional abstraction code in exchange for dramatically faster, more reliable tests
- **Flexibility vs. simplicity**: The trait allows easy substitution of filesystem backends (useful for future features like remote storage) at the cost of indirection

### Neutral

- Aligns with Rust ecosystem patterns (e.g., `std::io::Read`, `std::io::Write` traits)
- Follows dependency inversion principle from clean architecture
- Pattern is well-documented and familiar to Rust developers
