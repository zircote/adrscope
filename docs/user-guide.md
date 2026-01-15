# User Guide

Complete reference for ADRScope commands and features.

## Commands Overview

| Command | Description |
|---------|-------------|
| `generate` | Generate a self-contained HTML viewer |
| `validate` | Validate ADRs against rules |
| `stats` | Display ADR statistics |
| `wiki` | Generate GitHub Wiki pages |

## Generate Command

Creates an interactive HTML viewer for your ADRs.

```bash
adrscope generate [OPTIONS]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--input` | `-i` | `docs/decisions` | Input directory containing ADRs |
| `--output` | `-o` | `adr-viewer.html` | Output HTML file path |
| `--pattern` | `-p` | `**/*.md` | Glob pattern for finding ADR files |
| `--theme` | `-t` | `system` | Theme: `light`, `dark`, or `system` |
| `--verbose` | `-v` | - | Enable verbose output |

### Examples

Basic usage with defaults:

```bash
adrscope generate
```

Custom input and output:

```bash
adrscope generate -i architecture/decisions -o docs/adr.html
```

Dark theme with verbose output:

```bash
adrscope generate --theme dark --verbose
```

Custom file pattern:

```bash
adrscope generate --pattern "ADR-*.md"
```

### Output

The generated HTML file is completely self-contained with embedded CSS and JavaScript. It requires no external dependencies and can be:

- Opened directly in any browser
- Hosted on any static file server
- Shared via email or file transfer
- Committed to your repository

![ADRScope Main View](_assets/main.png)

## Validate Command

Checks ADRs for required and recommended fields.

```bash
adrscope validate [OPTIONS]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--input` | `-i` | `docs/decisions` | Input directory containing ADRs |
| `--pattern` | `-p` | `**/*.md` | Glob pattern for finding ADR files |
| `--strict` | - | - | Fail on warnings (for CI/CD) |
| `--json` | - | - | Output results as JSON |
| `--verbose` | `-v` | - | Enable verbose output |

### Examples

Basic validation:

```bash
adrscope validate -i docs/decisions
```

Strict mode for CI/CD pipelines:

```bash
adrscope validate --strict
```

JSON output for tooling integration:

```bash
adrscope validate --json
```

### Validation Rules

**Required Fields** (errors if missing):

- `title` - ADR title
- `status` - Current status

**Recommended Fields** (warnings if missing):

- `description` - Brief summary
- `created` - Creation date
- `author` - Decision maker(s)
- `category` - Classification
- `tags` - Searchable keywords

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All ADRs valid |
| 1 | Validation errors found |
| 2 | Validation warnings found (with `--strict`) |

### CI/CD Integration

Add to your GitHub Actions workflow:

```yaml
- name: Validate ADRs
  run: adrscope validate --strict
```

## Stats Command

Displays statistics about your ADR collection.

```bash
adrscope stats [OPTIONS]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--input` | `-i` | `docs/decisions` | Input directory containing ADRs |
| `--pattern` | `-p` | `**/*.md` | Glob pattern for finding ADR files |
| `--format` | `-f` | `text` | Output format: `text`, `json`, or `markdown` |

### Examples

Basic statistics:

```bash
adrscope stats
```

JSON output:

```bash
adrscope stats --format json
```

Markdown for documentation:

```bash
adrscope stats --format markdown >> docs/adr-summary.md
```

### Output

Statistics include:

- Total ADR count
- Breakdown by status
- Breakdown by category
- Top tags
- Top authors
- Date range (oldest to newest)

## Wiki Command

Generates GitHub Wiki-compatible pages.

```bash
adrscope wiki [OPTIONS]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--input` | `-i` | `docs/decisions` | Input directory containing ADRs |
| `--output` | `-o` | `wiki/` | Output directory for wiki pages |
| `--pattern` | `-p` | `**/*.md` | Glob pattern for finding ADR files |

### Examples

Generate wiki pages:

```bash
adrscope wiki -i docs/decisions -o wiki/
```

### Generated Pages

The wiki generator creates:

- **Home.md** - Index of all ADRs
- **ADR-XXXX.md** - Individual ADR pages
- **Status-Index.md** - ADRs grouped by status
- **Category-Index.md** - ADRs grouped by category
- **Timeline.md** - Chronological view

## ADR Format

ADRScope uses the structured-MADR format with YAML frontmatter.

### Required Structure

```markdown
---
title: Short Decision Title
status: accepted
---

## Context

[Problem description]

## Decision

[What was decided]

## Consequences

[Results of the decision]
```

### Full Frontmatter

```yaml
---
title: Use PostgreSQL for Data Storage
description: Decision to use PostgreSQL as our primary database
type: adr
category: architecture
tags:
  - database
  - postgresql
  - storage
status: accepted
created: 2025-01-15
author: Architecture Team
project: backend
technologies:
  - postgresql
  - sql
audience:
  - developers
  - devops
related:
  - adr-0001.md
  - adr-0003.md
---
```

### Status Values

| Status | Description |
|--------|-------------|
| `proposed` | Under discussion (default) |
| `accepted` | Approved and in effect |
| `deprecated` | Should not be used for new work |
| `superseded` | Replaced by another ADR |

Unknown status values are handled gracefully with a warning and default to `proposed`.

### Body Sections

Recommended sections following MADR format:

- **Context** - Problem and forces at play
- **Decision** - The chosen solution
- **Consequences** - Positive and negative outcomes
- **Options Considered** (optional) - Alternatives evaluated
- **Decision Outcome** (optional) - Detailed rationale

## Themes

The HTML viewer supports three themes:

| Theme | Description |
|-------|-------------|
| `light` | Light background with dark text |
| `dark` | Dark background with light text |
| `system` | Follows OS preference (default) |

Set the theme during generation:

```bash
adrscope generate --theme dark
```

Users can also toggle themes in the viewer interface.

## Relationship Graphs

ADRs can declare relationships using the `related` field:

```yaml
related:
  - adr-0001.md
  - adr-0003.md
```

The viewer displays these as an interactive graph showing how decisions connect:

![ADR View with Graph](_assets/view-0.png)

## Filtering and Search

The faceted search panel allows filtering by multiple criteria:

![Filter Panel](_assets/filters.png)

Available filters:

- **Status** - proposed, accepted, deprecated, superseded
- **Category** - architecture, security, tooling, etc.
- **Tags** - any tags defined in frontmatter
- **Author** - decision makers
- **Project** - for multi-project repositories
- **Technologies** - tech stack references

## Library Usage

ADRScope can be used as a Rust library:

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);

    let options = GenerateOptions::new("docs/decisions")
        .with_output("adr-viewer.html")
        .with_theme("dark");

    let result = use_case.execute(&options)?;
    println!("Generated viewer with {} ADRs", result.adr_count);

    Ok(())
}
```

See the [API documentation](https://docs.rs/adrscope) for complete library reference.
