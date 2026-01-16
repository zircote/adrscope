# ADRScope

[![CI](https://github.com/zircote/adrscope/actions/workflows/ci.yml/badge.svg)](https://github.com/zircote/adrscope/actions/workflows/ci.yml)
[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-ADRScope-blue?logo=github)](https://github.com/marketplace/actions/adrscope)
[![Crates.io](https://img.shields.io/crates/v/adrscope.svg?logo=rust&logoColor=white)](https://crates.io/crates/adrscope)
[![Documentation](https://docs.rs/adrscope/badge.svg)](https://docs.rs/adrscope)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-dea584?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![cargo-deny](https://img.shields.io/badge/security-cargo--deny-blue?logo=rust&logoColor=white)](https://github.com/EmbarkStudios/cargo-deny)

A lightweight visualization tool for Architecture Decision Records (ADRs).

ADRScope generates self-contained HTML viewers for ADRs following the [structured-MADR](https://github.com/zircote/structured-madr) format. It supports faceted search, relationship graphs, and GitHub Wiki generation.

## GitHub Action

Use ADRScope in your CI/CD pipelines to validate ADRs and generate documentation automatically.

```yaml
- name: Validate ADRs
  uses: zircote/adrscope@v0
  with:
    command: validate
    input-dir: docs/decisions
    strict: true
```

### Action Features

- **All commands available**: `validate`, `generate`, `stats`, `wiki`
- **Inline annotations**: Validation errors appear directly in PR diffs
- **Cross-platform**: Linux, macOS, Windows (x86_64, ARM64)
- **Fast startup**: Pre-built binaries (~2-5 seconds)

### Example Workflow

```yaml
name: ADR CI
on:
  pull_request:
    paths: ['docs/decisions/**']

jobs:
  adr:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Validate ADRs
        uses: zircote/adrscope@v0
        with:
          command: validate
          strict: true

      - name: Generate Viewer
        uses: zircote/adrscope@v0
        with:
          command: generate
          output: adr-viewer.html

      - uses: actions/upload-artifact@v4
        with:
          name: adr-viewer
          path: adr-viewer.html
```

### Publishing to GitHub Wiki

ADRScope can generate wiki pages and automatically publish them to your repository's GitHub Wiki.

**Generated Wiki Pages:**
- `ADR-Index.md` - Main index with table of all ADRs
- `ADR-By-Status.md` - ADRs grouped by status (proposed, accepted, deprecated, superseded)
- `ADR-By-Category.md` - ADRs grouped by category
- `ADR-Timeline.md` - Chronological timeline view
- `ADR-Statistics.md` - Summary statistics and breakdowns

**Complete Wiki Deployment Workflow:**

```yaml
name: Deploy ADRs to Wiki

on:
  push:
    branches: [main]
    paths:
      - 'docs/decisions/**'

jobs:
  deploy-wiki:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Generate Wiki Pages
        uses: zircote/adrscope@v0
        with:
          command: wiki
          input-dir: docs/decisions
          output: wiki/

      - name: Deploy to Wiki
        uses: Andrew-Chen-Wang/github-wiki-action@v4
        with:
          path: wiki/
```

**With Validation:**

Validate ADRs before deploying to wiki:

```yaml
name: Deploy ADRs to Wiki

on:
  push:
    branches: [main]
    paths:
      - 'docs/decisions/**'

jobs:
  deploy-wiki:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Validate ADRs
        uses: zircote/adrscope@v0
        with:
          command: validate
          strict: true

      - name: Generate Wiki Pages
        uses: zircote/adrscope@v0
        with:
          command: wiki
          output: wiki/

      - name: Deploy to Wiki
        uses: Andrew-Chen-Wang/github-wiki-action@v4
        with:
          path: wiki/
```

**Wiki Command Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `input-dir` | Directory containing ADR files | `docs/decisions` |
| `output` | Output directory for wiki files | `wiki/` |
| `pattern` | Glob pattern for ADR files | `**/*.md` |

📖 **[Full Action Documentation →](ACTION.md)**

## Screenshots

### Main Viewer
![ADRScope Main View](docs/_assets/main.png)

### ADR Details with Relationship Graph
| ADR Content View | Category Breakdown |
|:----------------:|:------------------:|
| ![ADR View](docs/_assets/view-0.png) | ![Categories](docs/_assets/categories.png) |

### Faceted Search & Filtering
| Filter Panel | Filtered Results |
|:------------:|:----------------:|
| ![Filters](docs/_assets/filters.png) | ![Filtered View](docs/_assets/view-1.png) |

## Features

- **Self-contained HTML viewer** - Single file with embedded CSS/JS, no dependencies
- **Faceted search** - Filter by status, category, tags, author, project, and technologies
- **Relationship graphs** - Interactive visualization of ADR relationships
- **Multiple themes** - Light, dark, and system-preference modes
- **GitHub Wiki generation** - Generate wiki pages with index, status, category, and timeline views
- **Validation** - Check ADRs for required and recommended fields
- **Statistics** - Analyze your ADR collection with detailed breakdowns
- **Lenient parsing** - Gracefully handles non-standard status values with warnings

## Installation

### GitHub Action (Recommended for CI/CD)

```yaml
- uses: zircote/adrscope@v0
  with:
    command: validate
```

See [ACTION.md](ACTION.md) for full documentation.

### Homebrew (macOS/Linux)

```bash
brew install zircote/tap/adrscope
```

### From crates.io

```bash
cargo install adrscope
```

### From source

```bash
git clone https://github.com/zircote/adrscope.git
cd adrscope
make install
```

## Quick Start

```bash
# Generate an HTML viewer from ADRs in docs/decisions
adrscope generate -i docs/decisions -o adr-viewer.html

# Validate ADRs (useful for CI/CD)
adrscope validate -i docs/decisions --strict

# Show statistics
adrscope stats -i docs/decisions

# Generate GitHub Wiki pages
adrscope wiki -i docs/decisions -o wiki/
```

## Documentation

- [GitHub Action Documentation](ACTION.md)
- [Getting Started Guide](docs/getting-started.md)
- [User Guide](docs/user-guide.md)
- [Configuration Reference](docs/configuration.md)
- [Architecture Decision Records](docs/decisions/)

## Commands

| Command | Description |
|---------|-------------|
| `generate` | Generate self-contained HTML viewer |
| `validate` | Validate ADRs against rules |
| `stats` | Show ADR statistics |
| `wiki` | Generate GitHub Wiki pages |

### Generate Options

```bash
adrscope generate [OPTIONS]

Options:
  -i, --input <DIR>     Input directory [default: docs/decisions]
  -o, --output <FILE>   Output HTML file [default: adr-viewer.html]
  -p, --pattern <GLOB>  File pattern [default: **/*.md]
  -t, --theme <THEME>   Theme: light, dark, system [default: system]
  -v, --verbose         Enable verbose output
```

### Validate Options

```bash
adrscope validate [OPTIONS]

Options:
  -i, --input <DIR>     Input directory [default: docs/decisions]
  -p, --pattern <GLOB>  File pattern [default: **/*.md]
  --strict              Fail on warnings (for CI/CD)
  --json                Output as JSON
  -v, --verbose         Enable verbose output
```

## ADR Format

ADRScope expects ADRs in the [zircote/structured-madr](https://github.com/zircote/structured-madr) format:

```markdown
---
title: Use PostgreSQL for Data Storage
description: Decision to use PostgreSQL as our primary database
status: accepted
category: architecture
tags:
  - database
  - postgresql
created: 2025-01-15
author: Architecture Team
related:
  - adr-0001.md
  - adr-0003.md
---

## Context

[Describe the context and problem...]

## Decision

[Describe the decision...]

## Consequences

[Describe the consequences...]
```

### Supported Status Values

| Status | Description |
|--------|-------------|
| `proposed` | Under discussion (default) |
| `accepted` | Approved and in effect |
| `deprecated` | Should not be used for new work |
| `superseded` | Replaced by another ADR |

Unknown status values are handled gracefully with a warning.

## Library Usage

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = GenerateUseCase::new(fs);
let options = GenerateOptions::new("docs/decisions")
    .with_output("adr-viewer.html");

let result = use_case.execute(&options)?;
println!("Generated viewer with {} ADRs", result.adr_count);
```

## Development

### Prerequisites

- Rust 1.85+ (2024 edition)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) for supply chain security

### Build Commands

```bash
make build      # Build debug binary
make release    # Build optimized binary
make test       # Run all tests
make lint       # Run clippy linter
make fmt        # Format code
make check      # Quick check (fmt + lint + test)
make ci         # Full CI pipeline
make install    # Install to ~/.cargo/bin
```

### Code Quality

- **Linting**: clippy with pedantic and nursery lints
- **Safety**: `#![forbid(unsafe_code)]`
- **Testing**: 180+ tests with 95%+ coverage
- **Supply Chain**: cargo-deny for dependency auditing

## MSRV Policy

The Minimum Supported Rust Version (MSRV) is **1.85**. Increasing the MSRV is considered a minor breaking change.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run checks (`make check`)
4. Commit with conventional commits (`git commit -m 'feat: add feature'`)
5. Push and open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [MADR](https://adr.github.io/madr/) - Markdown ADR format
- [ADR GitHub Organization](https://adr.github.io/) - ADR resources and tools
