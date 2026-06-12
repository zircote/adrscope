# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **[Docs]**: Add comprehensive dependencies reference documentation
  - Complete catalog of all external dependencies with purposes
  - Version constraints and update policy
  - Supply chain security information
  - Dependency graph visualization

- **[Attested Delivery]**: Release pipeline now attaches SLSA build provenance
  and a CycloneDX SBOM attestation to every platform binary, and fail-closed
  verifies every attestation before the GitHub Release is published
- **[Publish Workflow]**: crates.io publishing via OIDC Trusted Publishing,
  with SLSA provenance attested against the exact `.crate` bytes the registry
  serves
- **[Homebrew Workflow]**: Formula in `zircote/homebrew-tap` is regenerated
  automatically on release (source-built formula)
- **[CI]**: `pin-check` job asserts every workflow `uses:` reference is pinned
  to a full 40-char commit SHA
- **[SECURITY.md]**: Documents how to verify release artifact attestations
- **[LICENSE]**: Add MIT license text

### Changed

- **[Deps]**: Update pulldown-cmark from 0.13.0 to 0.13.1 (patch release)
- **[Release Artifacts]**: Binaries are now published as bare executables
  named `adrscope-{version}-{platform}` (e.g. `adrscope-0.4.0-linux-amd64`)
  instead of target-triple tar.gz/zip archives
- **[Action]**: Downloads the new artifact naming and fail-closed verifies
  the binary's attestation before use; falls back to legacy archives for
  releases <= 0.3.0
- **[Crate Packaging]**: `Cargo.toml` include allowlist keeps repo-internal
  files out of the published crate
>>>>>>> 28aebca (feat(release): adopt attested delivery pipeline)

## [0.3.0] - 2026-01-15

### Changed

- **[Action]**: Move action.yml to repository root for GitHub Marketplace publishing
- **[Docs]**: Add prominent GitHub Action section to README with examples
- **[Docs]**: Add Marketplace badge and Homebrew installation instructions

## [0.2.0] - 2026-01-15

### Added

- **[GitHub Action]**: Reusable composite action for CI/CD integration
  - All four commands exposed as action inputs (validate, generate, stats, wiki)
  - Problem matcher for inline PR annotations on validation errors/warnings
  - Cross-platform support (Linux, macOS, Windows on x86_64/aarch64)
  - Version pinning support with `latest` default
  - Binary download from GitHub Releases for fast startup (~2-5s)
- **[Release Workflow]**: Automated multi-platform binary builds
  - Builds for 5 targets: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)
  - Creates tar.gz archives for Unix, zip for Windows
  - SHA256 checksums generation
  - Automatic GitHub Release creation on tag push
- **[Homebrew]**: Formula for zircote/tap
  - macOS support (Intel and Apple Silicon)
  - Linux support (x86_64 and aarch64)
- **[CLI]**: Command-line interface with subcommands for generate, validate, stats, and wiki
- **[Generate]**: Self-contained HTML viewer generation with embedded CSS/JS
  - Faceted search and filtering by status, category, tags, author, and project
  - Interactive relationship graph visualization
  - Multiple theme support (light, dark, system)
  - Configurable glob patterns for ADR discovery
- **[Validate]**: ADR validation with configurable rules
  - Required fields validation (title, status)
  - Recommended fields warnings (description, created, category)
  - Strict mode for CI/CD pipelines
  - JSON output format for tooling integration
- **[Stats]**: Statistics and analytics for ADR collections
  - Status distribution breakdown
  - Category and tag frequency analysis
  - Author and project tracking
  - Date range reporting
  - Multiple output formats (text, JSON, markdown)
- **[Wiki]**: GitHub Wiki page generation
  - Index page with all ADRs
  - Status-based grouping
  - Category-based grouping
  - Timeline view
  - Statistics summary
- **[Domain]**: Structured-MADR frontmatter support
  - Full YAML frontmatter parsing with lenient validation
  - Status lifecycle states (proposed, accepted, deprecated, superseded)
  - Graceful handling of unknown status values with warnings
  - Relationship tracking via `related` field
  - Date parsing with ISO 8601 format
- **[Infrastructure]**: Robust parsing and rendering
  - Markdown to HTML rendering with syntax highlighting
  - Plain text extraction for search indexing
  - Relationship graph construction with edge types
  - Filesystem abstraction for testability

### Security

- **[Build]**: Unsafe code forbidden via `#![forbid(unsafe_code)]`
- **[Dependencies]**: Supply chain security via cargo-deny
  - License auditing (MIT, Apache-2.0, BSD only)
  - Vulnerability scanning via RustSec advisory database
  - Source verification (crates.io only)

## [0.1.0] - Unreleased

Initial release of ADRScope.

[Unreleased]: https://github.com/zircote/adrscope/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/zircote/adrscope/releases/tag/v0.3.0
[0.2.0]: https://github.com/zircote/adrscope/releases/tag/v0.2.0
[0.1.0]: https://github.com/zircote/adrscope/releases/tag/v0.1.0
