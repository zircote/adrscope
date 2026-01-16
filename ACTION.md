# ADRScope GitHub Action

Validate, visualize, and manage Architecture Decision Records (ADRs) in your GitHub workflows.

## Features

- **Validate ADRs** - Check for required fields, formatting, and best practices
- **Generate HTML Viewer** - Create an interactive, self-contained ADR browser
- **Statistics** - Get insights about your ADR collection
- **Wiki Export** - Generate GitHub Wiki-compatible pages
- **GitHub Annotations** - See validation issues inline in PR diffs

## Quick Start

```yaml
- name: Validate ADRs
  uses: zircote/adrscope@v0
  with:
    command: validate
    strict: true
```

## Usage

### Validate ADRs

```yaml
name: ADR Validation

on:
  pull_request:
    paths:
      - 'docs/decisions/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Validate ADRs
        uses: zircote/adrscope@v0
        with:
          command: validate
          input-dir: docs/decisions
          strict: true
```

### Generate HTML Viewer

```yaml
- name: Generate ADR Viewer
  uses: zircote/adrscope@v0
  with:
    command: generate
    input-dir: docs/decisions
    output: adr-viewer.html
    theme: auto

- name: Upload Viewer
  uses: actions/upload-artifact@v4
  with:
    name: adr-viewer
    path: adr-viewer.html
```

### Get Statistics

```yaml
- name: ADR Statistics
  uses: zircote/adrscope@v0
  with:
    command: stats
    format: markdown
```

### Generate Wiki Pages

```yaml
- name: Generate Wiki
  uses: zircote/adrscope@v0
  with:
    command: wiki
    output: wiki/
```

### Deploy to GitHub Wiki

Generate wiki pages and publish them to your repository's GitHub Wiki:

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
          input-dir: docs/decisions
          output: wiki/

      - name: Deploy to Wiki
        uses: Andrew-Chen-Wang/github-wiki-action@v4
        with:
          path: wiki/
```

**Generated Wiki Files:**
- `ADR-Index.md` - Table of all ADRs with ID, title, status, category, and date
- `ADR-By-Status.md` - ADRs grouped by status (proposed, accepted, deprecated, superseded)
- `ADR-By-Category.md` - ADRs organized by category
- `ADR-Timeline.md` - Chronological timeline of decisions
- `ADR-Statistics.md` - Summary statistics and breakdowns
- Individual ADR files are copied to the output directory

**Prerequisites:**
1. Enable the Wiki for your repository (Settings > Features > Wikis)
2. Create at least one wiki page manually (this initializes the wiki repository)

## Inputs

| Input | Description | Default |
|-------|-------------|---------|
| `command` | Command to run: `validate`, `generate`, `stats`, `wiki` | `validate` |
| `input-dir` | Directory containing ADR files | `docs/decisions` |
| `output` | Output file/directory path | Command-specific |
| `pattern` | Glob pattern for ADR files | `**/*.md` |
| `strict` | Treat warnings as errors (validate only) | `false` |
| `format` | Output format for stats: `text`, `json`, `markdown` | `text` |
| `theme` | Theme for generate: `light`, `dark`, `auto` | `auto` |
| `version` | ADRScope version to use | `latest` |
| `github-token` | Token for downloading releases | `${{ github.token }}` |

## Outputs

| Output | Description |
|--------|-------------|
| `passed` | Whether validation passed (`true`/`false`) |
| `error-count` | Number of validation errors |
| `warning-count` | Number of validation warnings |
| `output-file` | Path to generated file (generate/wiki) |

## GitHub Annotations

Validation errors and warnings appear as inline annotations in pull request diffs:

```
docs/decisions/adr-0001.md:1
  ⚠️ Missing recommended field: description [missing-field]
```

## ADR Format

ADRScope expects ADRs in the [zircote/structured-madr](https://github.com/zircote/structured-madr) format:

```markdown
---
title: Use PostgreSQL for Data Storage
status: accepted
description: Decision to use PostgreSQL
category: architecture
tags:
  - database
  - postgresql
created: 2025-01-15
author: Architecture Team
---

## Context

[Problem description]

## Decision

[What was decided]

## Consequences

[Results of the decision]
```

### Required Fields

- `title` - ADR title
- `status` - One of: `proposed`, `accepted`, `deprecated`, `superseded`

### Recommended Fields

- `description` - Brief summary
- `created` - Creation date (YYYY-MM-DD)
- `author` - Decision maker(s)
- `category` - Classification
- `tags` - Searchable keywords

## Examples

### Complete CI Workflow

```yaml
name: ADR CI

on:
  push:
    branches: [main]
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

      - name: Validate ADRs
        id: validate
        uses: zircote/adrscope@v0
        with:
          command: validate
          strict: true

      - name: Generate Viewer
        if: github.ref == 'refs/heads/main'
        uses: zircote/adrscope@v0
        with:
          command: generate
          output: adr-viewer.html

      - name: Upload to GitHub Pages
        if: github.ref == 'refs/heads/main'
        uses: actions/upload-pages-artifact@v3
        with:
          path: adr-viewer.html
```

### Matrix Testing

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: zircote/adrscope@v0
        with:
          command: validate
```

## Platform Support

| Platform | Architecture |
|----------|--------------|
| Linux | x86_64, aarch64 |
| macOS | x86_64, aarch64 |
| Windows | x86_64 |

## License

MIT License - see [LICENSE](../../../LICENSE) for details.
