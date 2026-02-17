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
    pub report: ValidationReport,
    pub adr_count: usize,
}

impl ValidateResult {
    pub fn has_errors(&self) -> bool;
    pub fn has_warnings(&self) -> bool;
    pub fn error_count(&self) -> usize;
    pub fn warning_count(&self) -> usize;
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
    pub id: AdrId,
    pub frontmatter: Frontmatter,
    pub markdown_body: String,
    pub html_body: String,
}
````

**Fields:**
- `id: AdrId` - Unique identifier derived from filename
- `frontmatter: Frontmatter` - Structured metadata from YAML frontmatter
- `markdown_body: String` - Original markdown content (without frontmatter)
- `html_body: String` - Rendered HTML content

**Example:**

````rust
use adrscope::domain::Adr;
use adrscope::infrastructure::parser::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let parser = DefaultAdrParser::new(fs);
let content = parser.read_file("docs/decisions/adr-0001.md")?;
let adr = parser.parse("adr-0001.md", &content)?;

println!("Title: {}", adr.frontmatter.title);
println!("Status: {:?}", adr.frontmatter.status);
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
    pub description: Option<String>,
    pub created: Option<String>,
    pub author: Option<String>,

    // Optional categorization
    pub r#type: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub project: Option<String>,

    // Optional context
    pub technologies: Vec<String>,
    pub audience: Vec<String>,
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
    pub issues: Vec<ValidationIssue>,
}

pub struct ValidationIssue {
    pub file: String,
    pub severity: Severity,
    pub message: String,
    pub line: Option<usize>,
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

pub trait ValidationRule: Send + Sync {
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue>;
}
````

**Example - Custom Validation Rule:**

````rust
use adrscope::domain::{Adr, ValidationIssue, ValidationRule, Severity};

struct CategoryRequiredRule;

impl ValidationRule for CategoryRequiredRule {
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        if adr.frontmatter.category.is_none() {
            return vec![ValidationIssue {
                file: adr.id.to_string(),
                severity: Severity::Error,
                message: "Category is required".to_string(),
                line: None,
            }];
        }
        vec![]
    }
}

use adrscope::domain::Validator;

let mut validator = Validator::new();
validator.add_rule(Box::new(CategoryRequiredRule));

let report = validator.validate(&adrs);
# Ok::<(), adrscope::Error>(())
````

## Infrastructure Layer

External system integrations and abstractions.

### FileSystem Trait

Abstraction for file I/O operations. Enables testing and custom storage backends.

````rust
pub trait FileSystem {
    fn read_to_string(&self, path: &str) -> Result<String>;
    fn write(&self, path: &str, content: &str) -> Result<()>;
    fn glob(&self, pattern: &str) -> Result<Vec<String>>;
    fn exists(&self, path: &str) -> bool;
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

struct CachedFileSystem {
    cache: HashMap<String, String>,
    backend: Box<dyn FileSystem>,
}

impl CachedFileSystem {
    fn new(backend: Box<dyn FileSystem>) -> Self {
        Self {
            cache: HashMap::new(),
            backend,
        }
    }
}

impl FileSystem for CachedFileSystem {
    fn read_to_string(&self, path: &str) -> Result<String> {
        if let Some(cached) = self.cache.get(path) {
            return Ok(cached.clone());
        }
        self.backend.read_to_string(path)
    }

    fn write(&self, path: &str, content: &str) -> Result<()> {
        self.backend.write(path, content)
    }

    fn glob(&self, pattern: &str) -> Result<Vec<String>> {
        self.backend.glob(pattern)
    }

    fn exists(&self, path: &str) -> bool {
        self.cache.contains_key(path) || self.backend.exists(path)
    }
}
````

### AdrParser Trait

Abstraction for parsing ADR files.

````rust
pub trait AdrParser {
    fn parse(&self, filename: &str, content: &str) -> Result<Adr>;
}
````

**Implementation:**
- `DefaultAdrParser` - Parses structured-MADR format

**Example - Using Parser:**

````rust
use adrscope::infrastructure::parser::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let parser = DefaultAdrParser::new(fs);

let content = std::fs::read_to_string("adr-0001.md")?;
let adr = parser.parse("adr-0001.md", &content)?;
# Ok::<(), adrscope::Error>(())
````

### HtmlRenderer

Generates self-contained HTML viewers.

````rust
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render(config: &RenderConfig, data: &ViewerData) -> Result<String>;
}

pub struct RenderConfig {
    pub title: String,
    pub theme: Theme,
}

pub enum Theme {
    Light,
    Dark,
    Auto,
}

pub struct ViewerData {
    pub adrs: Vec<Adr>,
    pub facets: Facets,
    pub graph: Graph,
}
````

**Example:**

````rust
use adrscope::infrastructure::renderer::{HtmlRenderer, RenderConfig, Theme, ViewerData};
use adrscope::domain::{Facets, Graph};

let config = RenderConfig {
    title: "My ADRs".to_string(),
    theme: Theme::Dark,
};

let data = ViewerData {
    adrs: vec![/* parsed ADRs */],
    facets: Facets::from_adrs(&adrs),
    graph: Graph::from_adrs(&adrs),
};

let html = HtmlRenderer::render(&config, &data)?;
# Ok::<(), adrscope::Error>(())
````

### WikiRenderer

Generates GitHub Wiki markdown pages.

````rust
pub struct WikiRenderer;

impl WikiRenderer {
    pub fn render(adrs: &[Adr]) -> Result<HashMap<String, String>>;
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

let pages = WikiRenderer::render(&adrs)?;

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
    use adrscope::infrastructure::fs::InMemoryFileSystem;
    use adrscope::application::{GenerateUseCase, GenerateOptions};

    #[test]
    fn test_generate() {
        let mut fs = InMemoryFileSystem::new();
        fs.add_file("docs/adr-0001.md", "---\ntitle: Test\nstatus: accepted\n---\n# Test");

        let use_case = GenerateUseCase::new(fs);
        let options = GenerateOptions::new("docs");

        let result = use_case.execute(&options).unwrap();
        assert_eq!(result.adr_count, 1);
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
    let options = GenerateOptions::new(&format!("docs/{}/decisions", project))
        .with_output(&format!("{}-adrs.html", project))
        .with_title(&format!("{} Architecture Decisions", project));

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
