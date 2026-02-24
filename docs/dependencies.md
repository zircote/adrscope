# Dependencies Reference

This document provides a complete reference of ADRScope's external dependencies, their purposes, and version constraints.

## Overview

ADRScope maintains a minimal dependency footprint while leveraging well-maintained ecosystem crates for specific functionality. All dependencies are audited for security and license compliance via `cargo-deny`.

## Dependency Categories

### Core Dependencies

Production dependencies used in the library and binary.

#### Command-Line Interface

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[clap](https://crates.io/crates/clap)** | 4.x | Command-line argument parsing | `derive` for proc-macro API |

**Why clap?** Industry-standard CLI framework with excellent ergonomics, comprehensive validation, and derive macros for reduced boilerplate.

#### Serialization & Data Formats

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[serde](https://crates.io/crates/serde)** | 1.x | Serialization/deserialization framework | `derive` for automatic trait implementation |
| **[serde_json](https://crates.io/crates/serde_json)** | 1.x | JSON serialization support | Default features |
| **[serde_yaml](https://crates.io/crates/serde_yaml)** | 0.9.x | YAML frontmatter parsing | Default features |

**Why these?** Serde is the de facto serialization standard in Rust. YAML support is essential for parsing ADR frontmatter. JSON support enables machine-readable output formats.

#### Templating

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[askama](https://crates.io/crates/askama)** | 0.14.x | Compile-time HTML template rendering | Default features |

**Why askama?** Compile-time template compilation provides zero runtime overhead, type-safe templates, and automatic HTML escaping for XSS prevention. See [ADR-0007](decisions/adr-0007-askama-compile-time-templates.md).

#### Markdown Processing

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[pulldown-cmark](https://crates.io/crates/pulldown-cmark)** | 0.13.x | CommonMark-compliant Markdown parsing | `html` for HTML rendering (disabled default features) |

**Why pulldown-cmark?** Fast, CommonMark-compliant parser with minimal allocations. Widely used and actively maintained. Handles edge cases in Markdown syntax correctly.

**Version History:**
- v0.13.1 (current) - Latest stable release with bug fixes
- v0.13.0 - Previous version

#### Error Handling

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[thiserror](https://crates.io/crates/thiserror)** | 2.x | Ergonomic error type definitions | Default features |

**Why thiserror?** Reduces boilerplate for custom error types with automatic `Display` and `Error` trait implementations. See [ADR-0005](decisions/adr-0005-unified-error-types-with-thiserror.md).

#### Date & Time

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[time](https://crates.io/crates/time)** | 0.3.x | Date/time handling for ADR metadata | `serde`, `formatting`, `parsing`, `macros` |

**Why time?** Memory-safe, actively maintained, and provides ISO 8601 date parsing required for ADR frontmatter.

#### File Globbing

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| **[glob](https://crates.io/crates/glob)** | 0.3.x | Pattern-based file discovery | Default features |

**Why glob?** Simple, reliable glob pattern matching for finding ADR files in directory trees.

### Development Dependencies

Dependencies used only in tests and development.

| Crate | Version | Purpose |
|-------|---------|---------|
| **[proptest](https://crates.io/crates/proptest)** | 1.x | Property-based testing framework |
| **[tempfile](https://crates.io/crates/tempfile)** | 3.x | Temporary file/directory creation for tests |
| **[pretty_assertions](https://crates.io/crates/pretty_assertions)** | 1.x | Better assertion failure output with diffs |

**Note:** Most tests use the `InMemoryFileSystem` abstraction rather than actual temporary files for speed and isolation.

## Version Constraints

### Semantic Versioning Policy

- **Caret requirements** (`^1.0`): Accept compatible updates
  - `1.x` accepts `1.0.0` through `1.999.999` (no breaking changes)
  - `0.x` accepts `0.x.0` through `0.x.999` (may have breaking changes)
- **Exact versions**: None currently used
- **Version ranges**: None currently used

### MSRV (Minimum Supported Rust Version)

**Rust 1.85+ (2024 edition)** is required.

All dependencies are compatible with this MSRV. The project uses:
- Rust 2024 edition features
- Modern trait bounds
- `thiserror` 2.x (requires Rust 1.77+)

### Update Policy

Dependencies are updated via:
1. **Dependabot**: Automated pull requests for patch and minor updates
2. **Manual review**: Major version updates require careful evaluation
3. **Security updates**: Expedited review and merge for CVE fixes

## Supply Chain Security

All dependencies undergo security and license auditing via [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny).

### Security Checks

- **Vulnerability scanning**: RustSec Advisory Database integration
- **Source verification**: All dependencies must come from crates.io
- **Transitive dependency auditing**: Full dependency tree scanned

### License Compliance

**Allowed licenses:**
- MIT
- Apache-2.0
- BSD-2-Clause
- BSD-3-Clause

**Rejected licenses:**
- GPL/LGPL (copyleft restrictions)
- Proprietary licenses
- Unknown licenses

See [`deny.toml`](../deny.toml) for complete configuration.

## Rationale for Minimal Dependencies

ADRScope intentionally maintains a small dependency footprint to:

1. **Reduce attack surface**: Fewer dependencies = fewer potential vulnerabilities
2. **Improve build times**: Small dependency tree compiles faster
3. **Enhance maintainability**: Less code to audit and update
4. **Ensure stability**: Fewer breaking changes from upstream updates

### What We Don't Depend On

- **HTTP clients**: No network I/O in core library
- **Async runtimes**: All operations are synchronous
- **Database drivers**: File-based operations only
- **UI frameworks**: Terminal-only interface
- **Compression libraries**: HTML viewers are self-contained but uncompressed
- **Image processing**: No embedded image manipulation

## Dependency Graph

````
adrscope (binary + library)
├── clap [CLI parsing]
├── serde [serialization framework]
│   ├── serde_json [JSON output]
│   └── serde_yaml [YAML frontmatter]
├── askama [HTML templates]
├── pulldown-cmark [Markdown → HTML]
├── thiserror [error types]
├── time [date parsing]
└── glob [file discovery]

Dev dependencies (tests only):
├── proptest [property-based testing]
├── tempfile [temporary files]
└── pretty_assertions [test output]
````

## Adding New Dependencies

Before adding a new dependency, consider:

1. **Is it necessary?** Can the functionality be implemented in ~100 lines of code?
2. **Is it maintained?** Active development, recent releases, responsive maintainers?
3. **Is it secure?** No known CVEs, passes `cargo-deny` checks?
4. **Is it compatible?** Matches project MSRV and license requirements?
5. **Does it fit the architecture?** Respects layer boundaries (see [Architecture](architecture.md))?

For detailed contribution guidelines, see [CONTRIBUTING.md](../CONTRIBUTING.md).

## Checking Dependencies

### View All Dependencies

````bash
# Production dependencies
cargo tree --depth 1

# Include dev dependencies
cargo tree --depth 1 --all-features

# Check for outdated dependencies
cargo outdated
````

### Audit Security

````bash
# Run cargo-deny checks
cargo deny check

# Or use make target
make deny
````

### Update Dependencies

````bash
# Update to latest compatible versions
cargo update

# Update specific dependency
cargo update -p pulldown-cmark

# Check what would be updated
cargo update --dry-run
````

## Related Documentation

- [Architecture](architecture.md) - How dependencies fit into the layer architecture
- [ADR-0006: Cargo Deny Supply Chain Security](decisions/adr-0006-cargo-deny-supply-chain-security.md) - Security rationale
- [ADR-0007: Askama Compile-Time Templates](decisions/adr-0007-askama-compile-time-templates.md) - Template engine choice
- [ADR-0005: Unified Error Types with thiserror](decisions/adr-0005-unified-error-types-with-thiserror.md) - Error handling approach
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development workflow
- [`Cargo.toml`](../Cargo.toml) - Dependency declarations
- [`deny.toml`](../deny.toml) - Security audit configuration

## Changelog

| Date | Version | Change |
|------|---------|--------|
| 2026-02-24 | 0.3.0 | Updated pulldown-cmark 0.13.0 → 0.13.1 (patch release) |
| 2026-01-15 | 0.3.0 | Initial dependency reference documentation |

---

**Last Updated**: 2026-02-24  
**ADRScope Version**: 0.3.0
