# API Reference

Complete reference for using ADRScope as a Rust library.

## Overview

ADRScope is built on Clean Architecture principles with four main layers:

````text
cli/           → Command-line interface (optional)
application/   → Use cases (main API entry points)
domain/        → Core business entities
infrastructure/→ External system integrations
````

## Quick Start

Add ADRScope to your `Cargo.toml`:

````toml
[dependencies]
adrscope = "0.3"
````

Basic usage:

````rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

fn main() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new("docs/decisions")
        .with_output("adr-viewer.html");

    let result = use_case.execute(&options)?;
    println!("Generated viewer with {} ADRs", result.adr_count);
    Ok(())
}
````

## Application Layer

The application layer provides use cases that orchestrate domain logic and infrastructure services. These are your main API entry points.

### GenerateUseCase

Generates self-contained HTML viewers for ADRs.

**Generic Parameter:**
- `F: FileSystem` - Filesystem abstraction for I/O operations

**Methods:**
- `new(fs: F) -> Self` - Create a new use case instance
- `execute(&self, options: &GenerateOptions) -> Result<GenerateResult>` - Execute the use case

**Example:**

````rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;
use adrscope::infrastructure::renderer::Theme;

let fs = RealFileSystem::new();
let use_case = GenerateUseCase::new(fs);

let options = GenerateOptions::new("docs/decisions")
    .with_output("viewer.html")
    .with_title("My Architecture Decisions")
    .with_pattern("**/*.md")
    .with_theme(Theme::Dark);

let result = use_case.execute(&options)?;
println!("Generated {} ADRs", result.adr_count);
# Ok::<(), adrscope::Error>(())
````

**GenerateOptions Builder:**

| Method | Parameter | Default | Description |
|--------|-----------|---------|-------------|
| `new(input)` | `&str` | - | Input directory (required) |
| `with_output(path)` | `&str` | `"adrs.html"` | Output HTML file |
| `with_title(title)` | `&str` | `"Architecture Decision Records"` | Page title |
| `with_pattern(pattern)` | `&str` | `"**/*.md"` | Glob pattern for files |
| `with_theme(theme)` | `Theme` | `Theme::Auto` | Visual theme |

**GenerateResult:**

````rust
pub struct GenerateResult {
    pub adr_count: usize,
    pub output_path: String,
}
````

### ValidateUseCase

Validates ADRs against structured-MADR format rules.

**Generic Parameter:**
- `F: FileSystem` - Filesystem abstraction for I/O operations

**Methods:**
- `new(fs: F) -> Self` - Create a new use case instance
- `execute(&self, options: &ValidateOptions) -> Result<ValidateResult>` - Execute validation

**Example:**

````rust
use adrscope::application::{ValidateOptions, ValidateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = ValidateUseCase::new(fs);

let options = ValidateOptions::new("docs/decisions")
    .with_strict(true)
    .with_pattern("adr-*.md");

let result = use_case.execute(&options)?;

if result.has_errors() {
    eprintln!("Validation failed with {} errors", result.error_count());
    std::process::exit(1);
}

println!("✓ All ADRs valid");
# Ok::<(), adrscope::Error>(())
````

**ValidateOptions Builder:**

| Method | Parameter | Default | Description |
|--------|-----------|---------|-------------|
| `new(input)` | `&str` | - | Input directory (required) |
| `with_pattern(pattern)` | `&str` | `"**/*.md"` | Glob pattern for files |
| `with_strict(strict)` | `bool` | `false` | Treat warnings as errors |

**ValidateResult:**

````rust
pub struct ValidateResult {
    /// Validation reports for each successfully parsed file.
    pub reports: Vec<(std::path::PathBuf, ValidationReport)>,
    /// Files that failed to parse.
    pub parse_errors: Vec<(std::path::PathBuf, Error)>,
    /// Total number of validation errors across all files.
    pub total_errors: usize,
    /// Total number of validation warnings across all files.
    pub total_warnings: usize,
    /// Whether validation passed (no errors).
    pub passed: bool,
}
````

### StatsUseCase

Computes statistics over ADR collections.

**Generic Parameter:**
- `F: FileSystem` - Filesystem abstraction for I/O operations

**Methods:**
- `new(fs: F) -> Self` - Create a new use case instance
- `execute(&self, options: &StatsOptions) -> Result<StatsResult>` - Compute statistics

**Example:**

````rust
use adrscope::application::{StatsOptions, StatsUseCase, StatsFormat};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = StatsUseCase::new(fs);

let options = StatsOptions::new("docs/decisions")
    .with_format(StatsFormat::Json);

let result = use_case.execute(&options)?;
println!("{}", result.formatted_output);
# Ok::<(), adrscope::Error>(())
````

**StatsOptions Builder:**

| Method | Parameter | Default | Description |
|--------|-----------|---------|-------------|
| `new(input)` | `&str` | - | Input directory (required) |
| `with_pattern(pattern)` | `&str` | `"**/*.md"` | Glob pattern for files |
| `with_format(format)` | `StatsFormat` | `StatsFormat::Text` | Output format |

**StatsFormat Enum:**

````rust
pub enum StatsFormat {
    Text,     // Human-readable terminal output
    Json,     // JSON for tooling
    Markdown, // Markdown for documentation
}
````

### WikiUseCase

Generates GitHub Wiki-compatible markdown pages.

**Generic Parameter:**
- `F: FileSystem` - Filesystem abstraction for I/O operations

**Methods:**
- `new(fs: F) -> Self` - Create a new use case instance
- `execute(&self, options: &WikiOptions) -> Result<WikiResult>` - Generate wiki pages

**Example:**

````rust
use adrscope::application::{WikiOptions, WikiUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = WikiUseCase::new(fs);

let options = WikiOptions::new("docs/decisions")
    .with_output("wiki/");

let result = use_case.execute(&options)?;
println!("Generated {} wiki pages", result.page_count);
# Ok::<(), adrscope::Error>(())
````

**WikiOptions Builder:**

| Method | Parameter | Default | Description |
|--------|-----------|---------|-------------|
| `new(input)` | `&str` | - | Input directory (required) |
| `with_output(path)` | `&str` | `"wiki/"` | Output directory |
| `with_pattern(pattern)` | `&str` | `"**/*.md"` | Glob pattern for files |

## Domain Layer

Core business entities and logic. These types represent your ADR data.

### Adr

Represents a parsed Architecture Decision Record.

````rust
pub struct Adr {
    // Internal fields are private; use accessor methods instead.
    /* fields omitted */
}

impl Adr {
    pub fn id(&self) -> &AdrId { /* ... */ }
    pub fn frontmatter(&self) -> &Frontmatter { /* ... */ }
    pub fn body_markdown(&self) -> &str { /* ... */ }
    pub fn body_html(&self) -> &str { /* ... */ }
    pub fn filename(&self) -> &str { /* ... */ }
    pub fn source_path(&self) -> &PathBuf { /* ... */ }
}
````

**Accessors:**
- `fn id(&self) -> &AdrId` - Unique identifier derived from filename
- `fn frontmatter(&self) -> &Frontmatter` - Structured metadata from YAML frontmatter
- `fn body_markdown(&self) -> &str` - Original markdown content (without frontmatter)
- `fn body_html(&self) -> &str` - Rendered HTML content
- `fn filename(&self) -> &str` - Original filename
- `fn source_path(&self) -> &PathBuf` - Source file path

**Example:**

````rust
use adrscope::domain::Adr;
use adrscope::infrastructure::parser::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;
use std::path::Path;

let fs = RealFileSystem::new();
let parser = DefaultAdrParser::new();
let path = Path::new("docs/decisions/adr-0001.md");
let content = fs.read_to_string(path)?;
let adr = parser.parse(path, &content)?;

println!("Title: {}", adr.frontmatter().title);
println!("Status: {:?}", adr.frontmatter().status);
# Ok::<(), adrscope::Error>(())
````

### Frontmatter

YAML metadata following the structured-MADR format.

````rust
pub struct Frontmatter {
    // Required fields
    pub title: String,
    pub status: Status,

    // Recommended fields
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub author: String,

    // Optional metadata
    #[serde(rename = "type", default = "default_type")]
    pub doc_type: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub project: String,
    #[serde(default, with = "optional_date")]
    pub created: Option<time::Date>,
    #[serde(default, with = "optional_date")]
    pub updated: Option<time::Date>,

    // Optional context
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(default)]
    pub audience: Vec<String>,
    #[serde(default)]
    pub related: Vec<String>,
}
````

**Example YAML:**

````yaml
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
````

### Status

ADR lifecycle status enumeration.

````rust
pub enum Status {
    Proposed,    // Under discussion
    Accepted,    // Approved and active
    Deprecated,  // No longer recommended
    Superseded,  // Replaced by another ADR
}
````

**Parsing:**

Status values are parsed case-insensitively. Unknown values trigger a warning and default to `Proposed`.

````rust
use adrscope::domain::Status;

let status: Status = "accepted".parse()?;
assert_eq!(status, Status::Accepted);
# Ok::<(), adrscope::Error>(())
````

### Graph, Node, Edge

Relationship graph between ADRs.

````rust
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Node {
    pub id: String,
    pub title: String,
    pub status: Status,
}

pub struct Edge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

pub enum EdgeType {
    Related,    // Generic relationship
    Supersedes, // One ADR replaces another
}
````

**Example:**

````rust
use adrscope::domain::Graph;

let graph = Graph::from_adrs(&adrs);

for edge in &graph.edges {
    println!("{} -> {} ({:?})", edge.from, edge.to, edge.edge_type);
}
# Ok::<(), adrscope::Error>(())
````

### Facets

Aggregated filter dimensions for faceted search.

````rust
pub struct Facets {
    pub facets: Vec<Facet>,
}

pub struct Facet {
    pub name: String,
    pub values: Vec<FacetValue>,
}

pub struct FacetValue {
    pub value: String,
    pub count: usize,
}
````

**Example:**

````rust
use adrscope::domain::Facets;

let facets = Facets::from_adrs(&adrs);

for facet in &facets.facets {
    println!("{}:", facet.name);
    for value in &facet.values {
        println!("  {} ({})", value.value, value.count);
    }
}
# Ok::<(), adrscope::Error>(())
````

**Available Facets:**
- `status` - ADR lifecycle states
- `category` - Primary classification
- `tags` - Searchable keywords
- `author` - Decision makers
- `project` - Project identifiers
- `technologies` - Tech stack references

### Validation Types

Types for ADR validation.

````rust
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn new() -> Self;
    pub fn add_issue(&mut self, issue: ValidationIssue);
    pub fn issues(&self) -> &[ValidationIssue];
    pub fn error_count(&self) -> usize;
    pub fn warning_count(&self) -> usize;
    pub fn has_errors(&self) -> bool;
    pub fn is_valid(&self) -> bool;
}

pub struct ValidationIssue {
    pub severity: Severity,
    pub path: PathBuf,
    pub message: String,
    pub line: Option<usize>,
    pub rule: String,
}

impl ValidationIssue {
    pub fn error(path: PathBuf, message: impl Into<String>, rule: impl Into<String>) -> Self;
    pub fn warning(path: PathBuf, message: impl Into<String>, rule: impl Into<String>) -> Self;
}

pub enum Severity {
    Error,
    Warning,
}
````

**Validator and Rules:**

````rust
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    /// Creates a new validator with an initial set of rules.
    pub fn new(rules: Vec<Box<dyn ValidationRule>>) -> Self;

    /// Adds a rule to the validator.
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>);

    /// Validates a collection of ADRs using all configured rules.
    pub fn validate_all(&self, adrs: &[Adr]) -> ValidationReport;
}

pub trait ValidationRule: Send + Sync {
    /// Machine-readable rule identifier.
    fn name(&self) -> &str;

    /// Human-readable description of what the rule checks.
    fn description(&self) -> &str;

    /// Applies the rule to the given ADR, recording any issues in the report.
    fn validate(&self, adr: &Adr, report: &mut ValidationReport);
}
````

**Example - Custom Validation Rule:**

````rust
use adrscope::domain::{Adr, ValidationIssue, ValidationReport, ValidationRule, Severity};

struct CategoryRequiredRule;

impl ValidationRule for CategoryRequiredRule {
    fn name(&self) -> &str {
        "category_required"
    }

    fn description(&self) -> &str {
        "Ensures that each ADR has a category set."
    }

    fn validate(&self, adr: &Adr, report: &mut ValidationReport) {
        if adr.category().is_empty() {
            report.add_issue(ValidationIssue::error(
                adr.source_path().clone(),
                "Category is required",
                self.name(),
            ));
        }
    }
}

use adrscope::domain::Validator;

let rules: Vec<Box<dyn ValidationRule>> = vec![
    Box::new(CategoryRequiredRule),
];
let validator = Validator::new(rules);

let report = validator.validate_all(&adrs);
# Ok::<(), adrscope::Error>(())
````

## Infrastructure Layer

External system integrations and abstractions.

### FileSystem Trait

Abstraction for file I/O operations. Enables testing and custom storage backends.

````rust
pub trait FileSystem: Send + Sync {
    /// Reads the contents of a file as a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Writes string contents to a file, creating parent directories as needed.
    fn write(&self, path: &Path, contents: &str) -> Result<()>;

    /// Lists all files matching a glob pattern in a directory.
    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>>;

    /// Checks if a path exists.
    fn exists(&self, path: &Path) -> bool;

    /// Creates a directory and all parent directories.
    fn create_dir_all(&self, path: &Path) -> Result<()>;
}
````

**Implementations:**
- `RealFileSystem` - Standard filesystem I/O
- `InMemoryFileSystem` - In-memory for testing (requires `testing` feature)

**Example - Custom Implementation:**

````rust
use adrscope::infrastructure::FileSystem;
use adrscope::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

struct CachedFileSystem {
    cache: Arc<RwLock<HashMap<PathBuf, String>>>,
    backend: Box<dyn FileSystem>,
}

impl CachedFileSystem {
    fn new(backend: Box<dyn FileSystem>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            backend,
        }
    }
}

impl FileSystem for CachedFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        let cache = self.cache.read().unwrap();
        if let Some(cached) = cache.get(path) {
            return Ok(cached.clone());
        }
        drop(cache);
        
        let content = self.backend.read_to_string(path)?;
        
        let mut cache = self.cache.write().unwrap();
        cache.insert(path.to_path_buf(), content.clone());
        
        Ok(content)
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        self.backend.write(path, content)
    }

    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
        self.backend.glob(base, pattern)
    }

    fn exists(&self, path: &Path) -> bool {
        let cache = self.cache.read().unwrap();
        cache.contains_key(path) || self.backend.exists(path)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        self.backend.create_dir_all(path)
    }
}
````

### AdrParser Trait

Abstraction for parsing ADR files.

````rust
pub trait AdrParser: Send + Sync {
    /// Parses an ADR from file contents.
    fn parse(&self, path: &Path, content: &str) -> Result<Adr>;
}
````

**Implementation:**
- `DefaultAdrParser` - Parses structured-MADR format

**Example - Using Parser:**

````rust
use adrscope::infrastructure::parser::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;
use std::path::Path;

let fs = RealFileSystem::new();
let parser = DefaultAdrParser::new();

let path = Path::new("adr-0001.md");
let content = fs.read_to_string(path)?;
let adr = parser.parse(path, &content)?;
# Ok::<(), adrscope::Error>(())
````

### HtmlRenderer

Generates self-contained HTML viewers.

````rust
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn new() -> Self;
    
    /// Renders a collection of ADRs to a self-contained HTML viewer.
    pub fn render(
        &self,
        adrs: Vec<Adr>,
        source_dir: &str,
        config: &RenderConfig,
    ) -> Result<String>;
}

pub struct RenderConfig {
    pub title: String,
    pub theme: Theme,
    pub embed_assets: bool,
}

impl RenderConfig {
    pub fn new(title: impl Into<String>) -> Self;
    pub fn with_theme(self, theme: Theme) -> Self;
}

pub enum Theme {
    Light,
    Dark,
    Auto,
}

pub struct ViewerData {
    pub meta: ViewerMeta,
    pub records: Vec<Adr>,
    pub facets: Facets,
    pub graph: Graph,
}

pub struct ViewerMeta {
    pub generated: String,
    pub generator: String,
    pub schema_version: String,
    pub source_dir: String,
}
````

**Example:**

````rust
use adrscope::infrastructure::renderer::{HtmlRenderer, RenderConfig, Theme};
use adrscope::domain::{Facets, Graph};

let renderer = HtmlRenderer::new();
let config = RenderConfig::new("My ADRs")
    .with_theme(Theme::Dark);

let html = renderer.render(adrs, "docs/decisions", &config)?;
# Ok::<(), adrscope::Error>(())
````

### WikiRenderer

Generates GitHub Wiki markdown pages.

````rust
pub struct WikiRenderer;

impl WikiRenderer {
    pub fn new() -> Self;
    
    /// Renders all wiki pages for a collection of ADRs.
    pub fn render_all(
        &self,
        adrs: &[Adr],
        pages_url: Option<&str>,
    ) -> Result<Vec<(String, String)>>;
    
    /// Individual page renderers:
    pub fn render_index(&self, adrs: &[Adr], pages_url: Option<&str>) -> String;
    pub fn render_by_status(&self, adrs: &[Adr]) -> String;
    pub fn render_by_category(&self, adrs: &[Adr]) -> String;
    pub fn render_timeline(&self, adrs: &[Adr]) -> String;
    pub fn render_statistics(&self, stats: &AdrStatistics) -> String;
}
````

**Generated Pages:**
- `ADR-Index.md` - Main index with all ADRs
- `ADR-By-Status.md` - Grouped by status
- `ADR-By-Category.md` - Grouped by category
- `ADR-Timeline.md` - Chronological timeline
- `ADR-Statistics.md` - Summary statistics

**Example:**

````rust
use adrscope::infrastructure::renderer::WikiRenderer;

let renderer = WikiRenderer::new();
let pages = renderer.render_all(&adrs, Some("https://example.com"))?;

for (filename, content) in pages {
    println!("Generated: {}", filename);
    std::fs::write(format!("wiki/{}", filename), content)?;
}
# Ok::<(), adrscope::Error>(())
````

## Error Handling

ADRScope uses a unified error type with `thiserror`.

````rust
pub enum Error {
    FileRead { path: String, source: std::io::Error },
    FileWrite { path: String, source: std::io::Error },
    InvalidFrontmatter { file: String, message: String },
    YamlParse { source: serde_yaml::Error },
    MissingField { field: String },
    TemplateRender { source: askama::Error },
    NoAdrsFound { directory: String },
    ValidationFailed,
    InvalidFilename { filename: String },
    GlobPattern { pattern: String, source: glob::PatternError },
    DateParse { value: String },
    JsonSerialize { source: serde_json::Error },
    InvalidUtf8 { source: std::string::FromUtf8Error },
}

pub type Result<T> = std::result::Result<T, Error>;
````

**Error Handling Best Practices:**

````rust
use adrscope::{Error, Result};

fn process_adrs() -> Result<()> {
    // Use ? operator for automatic error propagation
    let adrs = load_adrs()?;

    // Pattern match for specific error handling
    match validate_adrs(&adrs) {
        Ok(report) => println!("Validated {} ADRs", report.adr_count),
        Err(Error::ValidationFailed) => {
            eprintln!("Validation failed");
            std::process::exit(1);
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
````

## Testing Utilities

When the `testing` feature is enabled, ADRScope provides testing utilities.

**Enable in Cargo.toml:**

````toml
[dev-dependencies]
adrscope = { version = "0.3", features = ["testing"] }
````

**InMemoryFileSystem:**

````rust
#[cfg(test)]
mod tests {
    use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;
    use adrscope::application::{GenerateUseCase, GenerateOptions};
    use std::path::Path;

    #[test]
    fn test_generate() {
        let fs = InMemoryFileSystem::new();
        fs.add_file(
            Path::new("docs/adr-0001.md"),
            "---\ntitle: Test\nstatus: accepted\n---\n# Test"
        );

        let use_case = GenerateUseCase::new(fs);
        let options = GenerateOptions::new("docs");

        let result = use_case.execute(&options).unwrap();
        assert!(result.adr_count > 0);
    }
}
````

## Advanced Examples

### Multi-Project Repository

Handle ADRs from multiple projects in a monorepo:

````rust
use adrscope::application::{GenerateUseCase, GenerateOptions};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();

// Generate viewer for each project
for project in ["frontend", "backend", "mobile"] {
    let use_case = GenerateUseCase::new(fs.clone());
    let input_dir = format!("docs/{}/decisions", project);
    let output_file = format!("{}-adrs.html", project);
    let title = format!("{} Architecture Decisions", project);
    
    let options = GenerateOptions::new(input_dir)
        .with_output(output_file)
        .with_title(title);

    use_case.execute(&options)?;
}
# Ok::<(), adrscope::Error>(())
````

### CI/CD Integration

Validate ADRs in a build pipeline:

````rust
use adrscope::application::{ValidateUseCase, ValidateOptions};
use adrscope::infrastructure::fs::RealFileSystem;

fn main() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let use_case = ValidateUseCase::new(fs);
    let options = ValidateOptions::new("docs/decisions")
        .with_strict(true);

    let result = use_case.execute(&options)?;

    if result.has_errors() {
        for issue in &result.report.issues {
            eprintln!("{}:{} - {}", issue.file, issue.severity, issue.message);
        }
        std::process::exit(1);
    }

    println!("✓ All {} ADRs valid", result.adr_count);
    Ok(())
}
````

### Custom Validation Pipeline

Combine built-in and custom validation rules:

````rust
use adrscope::domain::{Validator, ValidationRule, Adr, ValidationIssue, Severity};
use adrscope::infrastructure::parser::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;

// Custom rule: enforce tag count
struct MinimumTagsRule(usize);

impl ValidationRule for MinimumTagsRule {
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        if adr.frontmatter.tags.len() < self.0 {
            return vec![ValidationIssue {
                file: adr.id.to_string(),
                severity: Severity::Warning,
                message: format!("Expected at least {} tags", self.0),
                line: None,
            }];
        }
        vec![]
    }
}

fn main() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let parser = DefaultAdrParser::new(fs.clone());

    let files = fs.glob("docs/decisions/**/*.md")?;
    let mut adrs = Vec::new();
    for file in files {
        let content = fs.read_to_string(&file)?;
        adrs.push(parser.parse(&file, &content)?);
    }

    let mut validator = Validator::new();
    // Add default rules
    validator.add_rules(adrscope::domain::validation::default_rules());
    // Add custom rule
    validator.add_rule(Box::new(MinimumTagsRule(3)));

    let report = validator.validate(&adrs);
    println!("Found {} issues", report.issues.len());

    Ok(())
}
````

## See Also

- [User Guide](user-guide.md) - Command-line usage
- [Configuration Reference](configuration.md) - CLI configuration
- [Getting Started](getting-started.md) - Quick start guide
- [Architecture Decision Records](decisions/) - ADRScope's own ADRs
- [Rust API Docs](https://docs.rs/adrscope) - Generated documentation
