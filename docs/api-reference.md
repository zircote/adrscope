# API Reference

Complete reference for the ADRScope Rust library API.

## Overview

ADRScope exposes a library API for programmatic use. This allows you to integrate ADR processing, validation, and generation into your own Rust applications.

## Quick Start

Add ADRScope to your `Cargo.toml`:

```toml
[dependencies]
adrscope = "0.3"
```

Basic usage:

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::RealFileSystem;

fn main() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new("docs/decisions")
        .with_output("adr-viewer.html");

    let result = use_case.execute(&options)?;
    println!("Generated viewer with {} ADRs", result.adr_count);

    Ok(())
}
```

## Module Organization

| Module | Purpose |
|--------|---------|
| [`application`](#application-module) | High-level use cases for commands |
| [`domain`](#domain-module) | Core business logic and data structures |
| [`infrastructure`](#infrastructure-module) | I/O, parsing, and rendering |
| [`cli`](#cli-module) | Command-line interface (for binary use) |
| [`error`](#error-module) | Error types and result aliases |

## Application Module

The application layer provides high-level use cases for each command.

### Generate Use Case

Generate self-contained HTML viewers for ADRs.

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = GenerateUseCase::new(fs);

let options = GenerateOptions::new("docs/decisions")
    .with_output("viewer.html")
    .with_title("My ADRs")
    .with_pattern("**/*.md");

let result = use_case.execute(&options)?;
println!("Generated {} ADRs", result.adr_count);
```

**Types**:
- `GenerateUseCase<F: FileSystem>` - Use case struct
- `GenerateOptions` - Configuration options (builder pattern)
- `GenerateResult` - Execution results

**Methods**:

#### `GenerateOptions::new(input: impl Into<String>) -> Self`

Create options with input directory. Default output: `adrs.html`

#### `GenerateOptions::with_output(self, output: impl Into<String>) -> Self`

Set output file path. Default: `adrs.html`

#### `GenerateOptions::with_title(self, title: impl Into<String>) -> Self`

Set HTML page title. Default: `"Architecture Decision Records"`

#### `GenerateOptions::with_pattern(self, pattern: impl Into<String>) -> Self`

Set glob pattern for finding ADR files. Default: `**/*.md`

#### `GenerateOptions::with_theme(self, theme: Theme) -> Self`

Set viewer theme (`Theme::Light`, `Theme::Dark`, `Theme::Auto`). Default: `Theme::Auto`

#### `GenerateUseCase::execute(&self, options: &GenerateOptions) -> Result<GenerateResult>`

Execute the generation use case.

**Returns**: `GenerateResult` with fields:
- `output_path: String` - Path to the generated file
- `adr_count: usize` - Number of ADRs included
- `parse_errors: Vec<(PathBuf, Error)>` - Files that failed to parse

### Validate Use Case

Validate ADRs against a set of rules.

```rust
use adrscope::application::{ValidateOptions, ValidateUseCase};
use adrscope::infrastructure::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = ValidateUseCase::new(fs);

let options = ValidateOptions::new("docs/decisions")
    .with_strict(true);

let result = use_case.execute(&options)?;
if result.passed {
    println!("All ADRs are valid!");
} else {
    println!("Found {} errors, {} warnings", result.total_errors, result.total_warnings);
}
```

**Types**:
- `ValidateUseCase<F: FileSystem>`
- `ValidateOptions`
- `ValidateResult`

**Methods**:

#### `ValidateOptions::new(input: impl Into<String>) -> Self`

Create options with input directory.

#### `ValidateOptions::with_strict(self, strict: bool) -> Self`

Enable strict mode (warnings treated as errors). Default: `false`

#### `ValidateOptions::with_pattern(self, pattern: impl Into<String>) -> Self`

Set glob pattern. Default: `**/*.md`

#### `ValidateUseCase::execute(&self, options: &ValidateOptions) -> Result<ValidateResult>`

Execute validation.

**Returns**: `ValidateResult` with fields:
- `reports: Vec<(PathBuf, ValidationReport)>` - Per-file validation reports
- `parse_errors: Vec<(PathBuf, Error)>` - Files that failed to parse
- `total_errors: usize` - Total error count across all files
- `total_warnings: usize` - Total warning count across all files
- `passed: bool` - Whether validation passed

### Stats Use Case

Compute statistics for ADRs.

```rust
use adrscope::application::{StatsOptions, StatsUseCase};
use adrscope::infrastructure::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = StatsUseCase::new(fs);

let options = StatsOptions::new("docs/decisions");
let result = use_case.execute(&options)?;

println!("Total ADRs: {}", result.statistics.total_count);
for (status, count) in &result.statistics.by_status {
    println!("  {}: {}", status, count);
}
```

**Types**:
- `StatsUseCase<F: FileSystem>`
- `StatsOptions`
- `StatsResult`
- `StatsFormat` - Output format enum (`Text`, `Json`, `Markdown`)

**Key Fields in `StatsResult`**:
- `statistics: AdrStatistics` - Computed statistics
- `output: String` - Formatted output string
- `parse_errors: Vec<(PathBuf, Error)>` - Files that failed to parse

**Key Fields in `AdrStatistics`**:
- `total_count: usize` - Total number of ADRs
- `by_status: HashMap<String, usize>` - Count per status
- `by_category: HashMap<String, usize>` - Count per category
- `by_author: HashMap<String, usize>` - Count per author
- `by_tag: HashMap<String, usize>` - Count per tag
- `by_technology: HashMap<String, usize>` - Count per technology
- `by_year: HashMap<i32, usize>` - Count per year
- `earliest_date: Option<Date>` - Earliest ADR date
- `latest_date: Option<Date>` - Latest ADR date

### Wiki Use Case

Generate GitHub Wiki pages from ADRs.

```rust
use adrscope::application::{WikiOptions, WikiUseCase};
use adrscope::infrastructure::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = WikiUseCase::new(fs);

let options = WikiOptions::new("docs/decisions")
    .with_output_dir("wiki-output");
let result = use_case.execute(&options)?;

println!("Generated {} wiki pages", result.generated_files.len());
```

**Types**:
- `WikiUseCase<F: FileSystem>`
- `WikiOptions`
- `WikiResult`

**Methods**:

#### `WikiOptions::new(input: impl Into<String>) -> Self`

Create options with input directory. Use builder methods for other settings.

#### `WikiOptions::with_output_dir(self, dir: impl Into<String>) -> Self`

Set output directory. Default: `wiki`

#### `WikiOptions::with_pages_url(self, url: impl Into<String>) -> Self`

Set GitHub Pages URL for cross-linking.

**Generated Pages**:
- `ADR-Index.md` - Main index
- `ADR-By-Status.md` - Grouped by status
- `ADR-By-Category.md` - Grouped by category
- `ADR-Timeline.md` - Chronological view
- `ADR-Statistics.md` - Statistics summary

## Domain Module

Core business logic with no I/O or infrastructure dependencies.

### ADR Structure

The `Adr` struct has private fields accessed via methods:

```rust
use adrscope::domain::{Adr, AdrId, Frontmatter, Status};

// Adr fields are private; use accessor methods:
// adr.id()           -> &AdrId
// adr.filename()     -> &str
// adr.source_path()  -> &PathBuf
// adr.frontmatter()  -> &Frontmatter
// adr.body_markdown() -> &str
// adr.body_html()    -> &str
// adr.body_text()    -> &str

// Convenience delegates from frontmatter:
// adr.title()        -> &str
// adr.description()  -> &str
// adr.status()       -> Status
// adr.category()     -> &str
// adr.tags()         -> &[String]
// adr.author()       -> &str
// adr.created()      -> Option<time::Date>
// adr.updated()      -> Option<time::Date>
// adr.related()      -> &[String]
```

### Frontmatter

The `Frontmatter` struct has public fields:

```rust
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub doc_type: String,       // default: "adr"
    pub category: String,
    pub tags: Vec<String>,
    pub status: Status,
    pub created: Option<time::Date>,
    pub updated: Option<time::Date>,
    pub author: String,
    pub project: String,
    pub technologies: Vec<String>,
    pub audience: Vec<String>,
    pub related: Vec<String>,
}
```

Builder pattern via `Frontmatter::new(title)` with `.with_status()`, `.with_category()`, etc.

### Status

```rust
pub enum Status {
    Proposed,   // default
    Accepted,
    Deprecated,
    Superseded,
}
```

Implements `FromStr` (case-insensitive), `Display`, and `Default` (defaults to `Proposed`).

### Facets

Extract search facets from ADRs:

```rust
use adrscope::domain::Facets;

let facets = Facets::from_adrs(&adrs);
```

### Graph

Build relationship graphs between ADRs:

```rust
use adrscope::domain::Graph;

let graph = Graph::from_adrs(&adrs);
for edge in graph.edges() {
    println!("{} -> {}", edge.from, edge.to);
}
```

### Statistics

Compute ADR statistics:

```rust
use adrscope::domain::AdrStatistics;

let stats = AdrStatistics::from_adrs(&adrs);
println!("Total: {}", stats.total_count);
for (status, count) in &stats.by_status {
    println!("  {}: {}", status, count);
}
```

### Validation

Validate ADRs against rules:

```rust
use adrscope::domain::{Validator, default_rules, ValidationReport, Severity};

let validator = Validator::new(default_rules());
let report: ValidationReport = validator.validate_all(&adrs);

if report.is_valid() {
    println!("All ADRs valid!");
} else {
    for issue in report.issues() {
        println!("{}: {}", issue.severity, issue.message);
    }
}
```

**Validation Rule Trait**:
```rust
pub trait ValidationRule: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn validate(&self, adr: &Adr, report: &mut ValidationReport);
}
```

**Built-in Rules** (returned by `default_rules()`):
- `RequiredFieldsRule` - Ensures required frontmatter fields (title)
- `RecommendedFieldsRule` - Warns about missing recommended fields

**Severity Levels**:
- `Error` - Critical issues
- `Warning` - Non-critical issues

## Infrastructure Module

External concerns: file I/O, parsing, rendering.

### FileSystem Trait

All file I/O goes through this trait for testability:

```rust
use adrscope::infrastructure::{FileSystem, RealFileSystem};
use std::path::Path;

pub trait FileSystem: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, contents: &str) -> Result<()>;
    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
}

// Production use
let fs = RealFileSystem::new();
let content = fs.read_to_string(Path::new("adr.md"))?;
```

### In-Memory FileSystem (Testing)

For tests without disk I/O:

```rust
// Available under #[cfg(test)] or with feature = "testing"
use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;

#[test]
fn test_example() {
    let fs = InMemoryFileSystem::new();
    fs.add_file("test.md", "# Test ADR");

    let content = fs.read_to_string(Path::new("test.md")).unwrap();
    assert_eq!(content, "# Test ADR");
}
```

### Parser

Parse ADRs from Markdown files via the `AdrParser` trait:

```rust
use adrscope::infrastructure::{AdrParser, DefaultAdrParser};
use std::path::Path;

let parser = DefaultAdrParser::new();
let content = std::fs::read_to_string("adr.md")?;
let adr = parser.parse(Path::new("adr.md"), &content)?;

println!("Title: {}", adr.title());
println!("Status: {}", adr.status());
```

### Renderer

Render ADRs to HTML or Wiki format:

```rust
use adrscope::infrastructure::{HtmlRenderer, RenderConfig, Theme};

// HTML rendering
let renderer = HtmlRenderer::new();
let config = RenderConfig::new("My ADRs").with_theme(Theme::Dark);
let html = renderer.render(adrs, "docs/decisions", &config)?;
```

Wiki rendering (via `infrastructure::renderer::wiki::WikiRenderer`):

```rust
use adrscope::infrastructure::renderer::wiki::WikiRenderer;

let wiki = WikiRenderer::new();
let pages = wiki.render_all(&adrs, None)?;
for (filename, content) in pages {
    println!("Generated: {}", filename);
}
```

## Error Module

Unified error handling:

```rust
use adrscope::{Error, Result};

pub enum Error {
    FileRead { path: PathBuf, source: std::io::Error },
    FileWrite { path: PathBuf, source: std::io::Error },
    InvalidFrontmatter { path: PathBuf, message: String },
    YamlParse { path: PathBuf, source: serde_yaml::Error },
    MissingField { path: PathBuf, field: &'static str },
    TemplateRender { source: askama::Error },
    NoAdrsFound { path: PathBuf },
    ValidationFailed(usize),
    InvalidFilename(String),
    GlobPattern(String),
    DateParse { path: PathBuf, message: String },
    JsonSerialize(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

All library functions return `Result<T>` for fallible operations.

## Feature Flags

Currently, ADRScope has one feature flag:

- `testing` - Enables `InMemoryFileSystem` for testing (automatically enabled for test builds)

## Examples

### Custom Validation Rule

```rust
use adrscope::domain::{
    Adr, ValidationRule, ValidationReport, ValidationIssue, Severity,
};
use std::path::PathBuf;

struct MinimumContentLengthRule {
    min_length: usize,
}

impl ValidationRule for MinimumContentLengthRule {
    fn name(&self) -> &str {
        "minimum-content-length"
    }

    fn description(&self) -> &str {
        "Ensures ADR content meets minimum length"
    }

    fn validate(&self, adr: &Adr, report: &mut ValidationReport) {
        if adr.body_markdown().len() < self.min_length {
            report.add_issue(ValidationIssue::warning(
                adr.source_path().clone(),
                format!(
                    "Content length {} is below minimum {}",
                    adr.body_markdown().len(),
                    self.min_length
                ),
                self.name(),
            ));
        }
    }
}
```

### Filtering ADRs

```rust
use adrscope::domain::{Adr, Status};

fn filter_accepted(adrs: Vec<Adr>) -> Vec<Adr> {
    adrs.into_iter()
        .filter(|adr| matches!(adr.status(), Status::Accepted))
        .collect()
}

fn filter_by_category(adrs: Vec<Adr>, category: &str) -> Vec<Adr> {
    adrs.into_iter()
        .filter(|adr| adr.category() == category)
        .collect()
}
```

### Multi-Project Generation

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::RealFileSystem;

fn generate_multiple_viewers() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);

    for project in ["backend", "frontend", "infra"] {
        let options = GenerateOptions::new(format!("docs/{}/decisions", project))
            .with_output(format!("{}-adrs.html", project))
            .with_title(format!("{} Architecture Decisions", project));

        use_case.execute(&options)?;
    }

    Ok(())
}
```

## Further Reading

- [Getting Started Guide](getting-started.md)
- [User Guide](user-guide.md)
- [Architecture Documentation](architecture.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Full API Documentation on docs.rs](https://docs.rs/adrscope)
