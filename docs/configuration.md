# Configuration Reference

ADRScope is configured through command-line options. This document covers all configuration options and common usage patterns.

## Command-Line Configuration

All ADRScope behavior is controlled via command-line arguments. There are no configuration files to manage.

### Global Options

These options are available for all commands:

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Display help information |
| `--version` | `-V` | Display version information |
| `--verbose` | `-v` | Enable verbose output |

## Generate Configuration

### Input Options

```bash
adrscope generate \
  --input docs/decisions \
  --pattern "**/*.md"
```

| Option | Default | Description |
|--------|---------|-------------|
| `--input` | `docs/decisions` | Directory containing ADR files |
| `--pattern` | `**/*.md` | Glob pattern for finding files |

#### Pattern Examples

```bash
# All markdown files (default)
--pattern "**/*.md"

# Only numbered ADRs
--pattern "ADR-*.md"

# Specific prefix pattern
--pattern "adr-[0-9][0-9][0-9][0-9]-*.md"

# Multiple directories
--input docs/arch --pattern "decisions/**/*.md"
```

### Output Options

```bash
adrscope generate \
  --output build/adrs.html \
  --theme dark
```

| Option | Default | Description |
|--------|---------|-------------|
| `--output` | `adrs.html` | Output HTML file path |
| `--title` | `Architecture Decision Records` | Page title |
| `--theme` | `auto` | Visual theme selection |

#### Theme Options

| Value | Behavior |
|-------|----------|
| `light` | Light background, dark text |
| `dark` | Dark background, light text |
| `auto` | Follows operating system preference |

## Validate Configuration

### Validation Modes

```bash
# Standard validation (warnings are non-fatal)
adrscope validate

# Strict validation (warnings become errors)
adrscope validate --strict
```

| Option | Default | Description |
|--------|---------|-------------|
| `--strict` | `false` | Treat warnings as errors |

### Validation Rules

ADRScope validates against the [zircote/structured-madr](https://github.com/zircote/structured-madr) format:

**Required Fields** (always errors):

| Field | Description |
|-------|-------------|
| `title` | ADR title in frontmatter |
| `status` | Current status value |

**Recommended Fields** (warnings, errors with `--strict`):

| Field | Description |
|-------|-------------|
| `description` | Brief summary of the decision |
| `created` | Creation date (YYYY-MM-DD format) |
| `author` | Person or team who made the decision |
| `category` | Classification (architecture, security, etc.) |
| `tags` | List of searchable keywords |

## Stats Configuration

### Output Formats

```bash
# Human-readable text (default)
adrscope stats

# JSON for tooling
adrscope stats --format json

# Markdown for documentation
adrscope stats --format markdown
```

| Format | Use Case |
|--------|----------|
| `text` | Terminal display, human reading |
| `json` | CI/CD pipelines, tooling integration |
| `markdown` | Documentation generation |

## Wiki Configuration

### Output Structure

```bash
adrscope wiki \
  --input docs/decisions \
  --output wiki/
```

Generated file structure:

```
wiki/
├── Home.md              # Main index
├── ADR-0001.md          # Individual pages
├── ADR-0002.md
├── Status-Index.md      # By status
├── Category-Index.md    # By category
└── Timeline.md          # Chronological
```

## ADR Frontmatter Schema

### Complete Schema

```yaml
---
# Required
title: string           # Short decision title
status: string          # proposed|accepted|deprecated|superseded

# Recommended
description: string     # Brief summary (1-2 sentences)
created: date           # YYYY-MM-DD format
author: string          # Person or team name

# Optional - Categorization
type: string            # Usually "adr"
category: string        # Primary classification
tags: string[]          # Searchable keywords
project: string         # Project identifier

# Optional - Context
technologies: string[]  # Tech stack references
audience: string[]      # Target readers
related: string[]       # Related ADR filenames
---
```

### Status Values

| Value | Description | Display Color |
|-------|-------------|---------------|
| `proposed` | Under active discussion | Blue |
| `accepted` | Approved and active | Green |
| `deprecated` | No longer recommended | Yellow |
| `superseded` | Replaced by another ADR | Red |

**Unknown Status Handling**: ADRScope handles unknown status values gracefully. When an unrecognized status is encountered:

1. A warning is printed (once per unique unknown value)
2. The status defaults to `proposed`
3. Processing continues normally

This enables lenient parsing of ADRs from various sources with non-standard status conventions.

### Date Formats

The `created` field accepts dates in ISO 8601 format:

```yaml
created: 2025-01-15
```

## CI/CD Integration

### GitHub Actions

```yaml
name: ADR Validation

on:
  push:
    paths:
      - 'docs/decisions/**'
  pull_request:
    paths:
      - 'docs/decisions/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install ADRScope
        run: cargo install adrscope

      - name: Validate ADRs
        run: adrscope validate --strict

      - name: Generate Viewer
        run: adrscope generate -o adr-viewer.html

      - name: Upload Viewer
        uses: actions/upload-artifact@v4
        with:
          name: adr-viewer
          path: adr-viewer.html
```

### Pre-commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Validate ADRs before commit
if git diff --cached --name-only | grep -q "docs/decisions/"; then
    echo "Validating ADRs..."
    adrscope validate --strict
    if [ $? -ne 0 ]; then
        echo "ADR validation failed. Please fix errors before committing."
        exit 1
    fi
fi
```

## Environment Variables

ADRScope does not currently use environment variables for configuration. All settings are provided via command-line arguments.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (validation errors, file not found, etc.) |
| 2 | Warning (validation warnings with `--strict`) |

## Best Practices

### Directory Structure

Recommended project layout:

```
project/
├── docs/
│   ├── decisions/           # ADR storage
│   │   ├── README.md        # ADR index
│   │   ├── adr-0001-*.md
│   │   └── adr-0002-*.md
│   ├── getting-started.md
│   └── user-guide.md
├── .gitignore               # Ignore generated files
└── adr-viewer.html          # Generated (gitignored)
```

### Naming Convention

Use consistent naming for ADR files:

```
adr-NNNN-short-title.md
```

Examples:

- `adr-0001-use-postgresql.md`
- `adr-0002-adopt-clean-architecture.md`
- `adr-0003-forbid-unsafe-code.md`

### Gitignore

Add generated files to `.gitignore`:

```gitignore
# Generated ADR viewer
adr-viewer.html
adr.html
```

Or commit the viewer if you want it in the repository for easy access.
