# ADRScope Project Specification

## Overview

**Project**: ADRScope  
**Author**: Robert Allen (@zircote)  
**Repository**: `zircote/adrscope`  
**License**: MIT  

ADRScope is a lightweight, zero-dependency visualization tool for Architecture Decision Records (ADRs) following the [structured-madr](https://github.com/zircote/structured-madr) specification. It generates a single self-contained HTML file providing interactive filtering, sorting, searching, and relationship visualization of project ADRs.

---

## Goals

1. **Zero runtime dependencies** — Single HTML file, no server required
2. **Makefile integration** — `make adr-scope` generates and opens viewer
3. **Structured-MADR native** — First-class support for the structured-madr schema
4. **Offline-capable** — Works without network access
5. **CI/CD friendly** — Generate artifacts for GitHub Pages and wiki sync

---

## Non-Goals

- Real-time collaboration or editing
- ADR authoring or modification
- Database or backend persistence
- Framework-specific implementations (React, Vue, etc.)

---

## Schema Reference

ADRScope consumes ADRs with YAML frontmatter conforming to structured-madr v1.0:

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Short descriptive title (1-100 chars) |
| `description` | string | One-sentence summary (1-300 chars) |
| `type` | string | Document type identifier (const: `"adr"`) |
| `category` | string | Decision category (e.g., architecture, api, security) |
| `tags` | array[string] | Keywords for categorization (kebab-case) |
| `status` | enum | `proposed`, `accepted`, `deprecated`, `superseded` |
| `created` | date | ISO 8601 date created |
| `updated` | date | ISO 8601 date last modified |
| `author` | string | Author or team responsible |
| `project` | string | Project this decision applies to |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `technologies` | array[string] | Technologies affected by decision |
| `audience` | array[string] | Intended readers |
| `related` | array[string] | Filenames of related ADRs (e.g., `["adr_0001.md"]`) |
| `x-*` | any | Custom extension fields |

---

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────┐
│                      adrscope CLI                       │
├─────────────────────────────────────────────────────────┤
│  Parser          │  Renderer        │  Output           │
│  ─────────────   │  ────────────    │  ──────────       │
│  - YAML front-   │  - Markdown→HTML │  - HTML (Pages)   │
│    matter        │  - Template      │  - Markdown       │
│  - Markdown      │    engine        │    (Wiki)         │
│    body          │  - Asset embed   │  - JSON (API)     │
│  - Validation    │                  │                   │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

```
docs/decisions/*.md
        │
        ▼
┌───────────────────┐
│   Parse Phase     │
│   - Read files    │
│   - Extract YAML  │
│   - Parse MD body │
│   - Validate      │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│   Build Phase     │
│   - Render MD→HTML│
│   - Compute facets│
│   - Build index   │
│   - Resolve links │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│   Output Phase    │
│   - Embed data    │
│   - Apply template│
│   - Write files   │
└───────────────────┘
```

---

## CLI Specification

### Installation

```bash
# Cargo (primary)
cargo install adrscope

# Homebrew
brew install zircote/tap/adrscope

# From source
git clone https://github.com/zircote/adrscope
cd adrscope
cargo install --path .
```

### Commands

#### `adrscope generate`

Generate the interactive HTML viewer.

```bash
adrscope generate [OPTIONS]
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--input`, `-i` | path | `docs/decisions` | ADR source directory |
| `--output`, `-o` | path | `ADRScope.html` | Output file path |
| `--title` | string | `"ADR Explorer"` | Page title |
| `--theme` | enum | `auto` | `light`, `dark`, `auto` |
| `--open` | flag | false | Open in browser after generation |
| `--validate` | flag | true | Validate against schema |
| `--strict` | flag | false | Fail on validation warnings |

**Examples**:

```bash
# Basic usage
adrscope generate

# Custom paths with auto-open
adrscope generate -i ./adrs -o build/decisions.html --open

# CI mode (strict validation, no open)
adrscope generate --strict --output dist/ADRScope.html
```

#### `adrscope wiki`

Generate static markdown files for GitHub Wiki.

```bash
adrscope wiki [OPTIONS]
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--input`, `-i` | path | `docs/decisions` | ADR source directory |
| `--output`, `-o` | path | `wiki/` | Output directory |
| `--index-only` | flag | false | Generate only index pages |
| `--pages-url` | string | none | URL to interactive viewer (for linking) |

**Output Structure**:

```
wiki/
├── ADR-Index.md              # Master index table
├── ADR-By-Status.md          # Grouped by status
├── ADR-By-Category.md        # Grouped by category
├── ADR-Timeline.md           # Chronological listing
└── decisions/
    ├── adr_0001.md           # Individual ADRs (optional sync)
    └── ...
```

#### `adrscope validate`

Validate ADRs against the structured-madr schema.

```bash
adrscope validate [OPTIONS]
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--input`, `-i` | path | `docs/decisions` | ADR source directory |
| `--strict` | flag | false | Treat warnings as errors |
| `--format` | enum | `text` | `text`, `json`, `github` |

**Exit Codes**:

| Code | Meaning |
|------|---------|
| 0 | All valid |
| 1 | Validation errors |
| 2 | File/parse errors |

#### `adrscope stats`

Print summary statistics to terminal.

```bash
adrscope stats [OPTIONS]
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--input`, `-i` | path | `docs/decisions` | ADR source directory |
| `--format` | enum | `text` | `text`, `json` |

**Example Output**:

```
ADR Statistics
══════════════
Total:       24 records
By Status:   accepted (18), proposed (4), deprecated (2)
By Category: architecture (10), api (6), security (4), infrastructure (4)
Authors:     Architecture Team (15), Jane Doe (6), Platform Team (3)
Date Range:  2024-03-15 → 2025-01-15
```

---

## HTML Viewer Specification

### Embedded Data Structure

```javascript
const ADRSCOPE_DATA = {
  meta: {
    generated: "2025-01-15T10:30:00Z",
    generator: "adrscope/0.1.0",
    schema_version: "1.0.0",
    source_dir: "docs/decisions"
  },
  records: [
    {
      id: "adr_0001",
      filename: "adr_0001.md",
      frontmatter: {
        title: "Use PostgreSQL for Primary Storage",
        description: "Decision to adopt PostgreSQL...",
        type: "adr",
        category: "architecture",
        tags: ["database", "postgresql", "storage"],
        status: "accepted",
        created: "2025-01-15",
        updated: "2025-01-20",
        author: "Architecture Team",
        project: "my-application",
        technologies: ["postgresql", "rust"],
        audience: ["developers", "architects"],
        related: ["adr_0005.md"]
      },
      body_html: "<h2>Context</h2><p>...</p>",
      body_text: "Context\n\nWe need a primary database..."
    }
    // ... more records
  ],
  facets: {
    statuses: [
      { value: "accepted", count: 18 },
      { value: "proposed", count: 4 },
      { value: "deprecated", count: 2 },
      { value: "superseded", count: 0 }
    ],
    categories: [
      { value: "architecture", count: 10 },
      { value: "api", count: 6 }
      // ...
    ],
    tags: [
      { value: "database", count: 5 },
      { value: "postgresql", count: 3 }
      // ...
    ],
    authors: [
      { value: "Architecture Team", count: 15 }
      // ...
    ],
    projects: [
      { value: "my-application", count: 20 }
      // ...
    ],
    technologies: [
      { value: "postgresql", count: 4 }
      // ...
    ]
  },
  graph: {
    nodes: [
      { id: "adr_0001", status: "accepted" },
      { id: "adr_0005", status: "superseded" }
    ],
    edges: [
      { source: "adr_0001", target: "adr_0005", type: "related" }
    ]
  }
};
```

### UI Components

#### Header

- Page title (configurable)
- Search input (global full-text)
- View mode toggle (List | Cards | Timeline | Graph)
- Theme toggle (Light | Dark | Auto)

#### Sidebar (Filters)

- **Status**: Multi-select chips with color coding
  - `proposed` → yellow/amber
  - `accepted` → green
  - `deprecated` → red
  - `superseded` → gray
- **Category**: Dropdown with counts
- **Project**: Dropdown (if multiple projects)
- **Author**: Dropdown with counts
- **Tags**: Tag cloud or multi-select (show top N, expandable)
- **Technologies**: Multi-select
- **Date Range**: From/To date pickers for `created` or `updated`
- **Clear All Filters** button

#### Main Content Area

##### List View (Default)

Sortable table with columns:

| Column | Sortable | Notes |
|--------|----------|-------|
| Status | ✓ | Color-coded badge |
| Title | ✓ | Link to detail view |
| Category | ✓ | Badge/chip |
| Author | ✓ | |
| Created | ✓ | Formatted date |
| Updated | ✓ | Formatted date, highlight if recent |

Click row → expand inline or navigate to detail panel.

##### Card View

Grid of cards showing:
- Status badge (top-right corner)
- Title (heading)
- Description (truncated)
- Category + top 3 tags (chips)
- Author + dates (footer)

##### Timeline View

Vertical timeline:
- Y-axis: time (created date)
- Nodes: ADR cards (compact)
- Color: status
- Grouping: by month/quarter
- Branching: show supersedes relationships

##### Graph View

Interactive network graph:
- Nodes: ADRs (sized by connection count)
- Node color: status
- Edges: `related` relationships
- Clustering: by category or project
- Interactions: hover for preview, click for detail

#### Detail Panel

Full ADR display when selected:
- Title + status badge
- Frontmatter metadata table
- Rendered markdown body
- Related ADRs (clickable links)
- Navigation: Previous | Next | Back to list

#### Footer

- Record count (filtered / total)
- Generation timestamp
- "Generated by ADRScope" attribution

### Search Behavior

- **Debounced input** (300ms delay)
- **Search targets** (weighted):
  1. `title` (weight: 3)
  2. `description` (weight: 2)
  3. `tags` (weight: 2)
  4. `technologies` (weight: 1)
  5. `body_text` (weight: 1)
- **Fuzzy matching** with typo tolerance
- **Highlight** matched terms in results

### Keyboard Navigation

| Key | Action |
|-----|--------|
| `/` | Focus search |
| `Escape` | Clear search / close detail |
| `↑` / `↓` | Navigate list |
| `Enter` | Open selected |
| `1-4` | Switch view modes |
| `?` | Show keyboard shortcuts |

### Responsive Design

| Breakpoint | Layout |
|------------|--------|
| Desktop (≥1024px) | Sidebar + main content |
| Tablet (768-1023px) | Collapsible sidebar |
| Mobile (<768px) | Filter drawer, single column |

### Accessibility

- Semantic HTML (`<nav>`, `<main>`, `<article>`, etc.)
- ARIA labels for interactive elements
- Keyboard navigable
- Sufficient color contrast (WCAG AA)
- Reduced motion support

---

## Implementation Plan

### Phase 1: Core CLI (MVP)

**Milestone**: Basic generation working end-to-end

| Task | Priority | Estimate |
|------|----------|----------|
| Project scaffolding (cargo, clippy, rustfmt) | P0 | 1h |
| Type definitions (serde structs) | P0 | 1h |
| YAML frontmatter parser | P0 | 2h |
| Markdown body parser (pulldown-cmark) | P0 | 1h |
| Schema validation | P0 | 2h |
| Basic HTML template (list view only) | P0 | 3h |
| `generate` command implementation (clap) | P0 | 2h |
| Makefile integration example | P0 | 30m |

**Deliverable**: `adrscope generate` produces working HTML with list view

### Phase 2: Full Viewer UI

**Milestone**: All view modes and filtering

| Task | Priority | Estimate |
|------|----------|----------|
| Filter sidebar implementation | P0 | 3h |
| Search with fuzzy matching | P0 | 2h |
| Card view | P1 | 2h |
| Timeline view | P1 | 3h |
| Graph view (relationship visualization) | P1 | 4h |
| Detail panel | P0 | 2h |
| Theme support (light/dark/auto) | P2 | 1h |
| Keyboard navigation | P2 | 2h |

**Deliverable**: Fully interactive viewer with all modes

### Phase 3: Wiki & CI Integration

**Milestone**: GitHub ecosystem integration

| Task | Priority | Estimate |
|------|----------|----------|
| `wiki` command implementation | P1 | 3h |
| Wiki markdown templates | P1 | 2h |
| `validate` command | P1 | 1h |
| `stats` command | P2 | 1h |
| GitHub Action workflow example | P1 | 1h |
| Homebrew formula | P2 | 1h |

**Deliverable**: Full CI/CD workflow with wiki sync

### Phase 4: Polish & Distribution

**Milestone**: Production-ready release

| Task | Priority | Estimate |
|------|----------|----------|
| Responsive design refinement | P1 | 2h |
| Accessibility audit | P1 | 2h |
| Performance optimization | P2 | 2h |
| Documentation site | P1 | 3h |
| crates.io publishing | P0 | 1h |
| cargo-dist release workflow | P0 | 2h |
| Homebrew tap formula | P1 | 30m |
| npm distribution (optional) | P2 | 1h |

**Deliverable**: v1.0.0 release

---

## Technical Stack

### CLI

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust (stable) | Performance, single binary distribution |
| CLI framework | clap | Standard, derive macros |
| YAML parsing | serde_yaml | Frontmatter extraction |
| Markdown parsing | pulldown-cmark | CommonMark compliant, fast |
| Schema validation | jsonschema (crate) or custom | Structured-madr validation |
| Templating | minijinja or askama | HTML generation |
| Serialization | serde + serde_json | Data embedding |
| Linting | clippy | Standard |
| Formatting | rustfmt | Standard |
| Testing | cargo test | Built-in |
| Distribution | cargo-dist, homebrew-tap | Binary releases |

### HTML Viewer (Embedded)

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Styling | Pico CSS (~10kb) or vanilla | Minimal footprint |
| Search | Fuse.js (~6kb minified) | Lightweight fuzzy search |
| Graph | vis-network or Cytoscape.js | If graph view included |
| Markdown | Pre-rendered server-side | Zero client-side deps |
| Charts | None initially | Keep it simple |

### Alternative: Zero External JS

Embed all logic as vanilla JS:
- Custom fuzzy search (~50 lines)
- CSS-only cards/grid
- SVG-based graph (pre-computed positions)

---

## File Structure

```
adrscope/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── Makefile
├── src/
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library root
│   ├── cli.rs                  # Clap CLI definitions
│   ├── parser.rs               # YAML/MD parsing
│   ├── validator.rs            # Schema validation
│   ├── renderer.rs             # HTML generation
│   ├── wiki.rs                 # Wiki markdown generation
│   ├── stats.rs                # Statistics computation
│   ├── types.rs                # Data structures
│   └── templates/
│       ├── mod.rs              # Template module
│       ├── viewer.html         # Main HTML template
│       ├── styles.css          # Embedded styles
│       ├── app.js              # Embedded JavaScript
│       └── wiki/
│           ├── index.md        # Wiki index template
│           ├── by_status.md    # By status template
│           ├── by_category.md  # By category template
│           └── timeline.md     # Timeline template
├── tests/
│   ├── integration.rs
│   └── fixtures/
│       └── decisions/
│           ├── adr_0001.md
│           └── adr_0002.md
├── benches/
│   └── generation.rs           # Performance benchmarks
└── docs/
    └── decisions/              # ADRScope's own ADRs
        └── 0001-use-rust.md
```

---

## Testing Strategy

### Unit Tests

- Parser: frontmatter extraction, edge cases (`#[cfg(test)]` modules)
- Validator: schema compliance, error messages
- Renderer: template output, data embedding

### Integration Tests

- End-to-end: source files → HTML output (`tests/` directory)
- Wiki generation: source files → markdown output
- CLI: argument parsing, option handling

### Benchmarks

- Generation performance (`benches/` with criterion)
- Parser throughput
- Large ADR set scaling

### Fixture ADRs

Include sample ADRs covering:
- All status values
- All optional fields present/absent
- Related ADR chains
- Extension fields (`x-*`)
- Edge cases (long titles, many tags, special characters)

---

## Success Criteria

| Criterion | Metric |
|-----------|--------|
| Generation time | <500ms for 100 ADRs |
| Binary size | <10MB (stripped) |
| Output size | <500kb for 100 ADRs (HTML) |
| Zero runtime errors | All browsers (Chrome, Firefox, Safari, Edge) |
| Accessibility | WCAG AA compliance |
| Test coverage | >80% |
| Memory usage | <50MB peak for 1000 ADRs |

---

## Future Considerations

Not in scope for v1.0, but potential future enhancements:

- **ADR authoring CLI** (`adrscope new`)
- **Git integration** (show diffs, blame info)
- **Multi-repo aggregation** (combine ADRs from multiple repos)
- **Export formats** (PDF, EPUB)
- **Embedding in docs sites** (Astro, MkDocs integration)
- **MCP server** (for Claude Code queries against ADRs)
- **VS Code extension** (preview panel)

---

## References

- [structured-madr schema](https://github.com/zircote/structured-madr)
- [MADR project](https://adr.github.io/madr/)
- [ADR GitHub organization](https://adr.github.io/)
- [git-adr](https://github.com/zircote/git-adr) (related project)
