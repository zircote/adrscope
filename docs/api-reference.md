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
use adrscope::infrastructure::fs::RealFileSystem;

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

#### `GenerateOptions::new(input: impl Into<PathBuf>) -> Self`

Create options with input directory.

#### `GenerateOptions::with_output(self, output: impl Into<PathBuf>) -> Self`

Set output file path. Default: `adrs.html`

#### `GenerateOptions::with_title(self, title: impl Into<String>) -> Self`

Set HTML page title. Default: `"Architecture Decision Records"`

#### `GenerateOptions::with_pattern(self, pattern: impl Into<String>) -> Self`

Set glob pattern for finding ADR files. Default: `**/*.md`

#### `GenerateUseCase::execute(&self, options: &GenerateOptions) -> Result<GenerateResult>`

Execute the generation use case.

**Returns**: `GenerateResult` with `adr_count` and `output_path`

### Validate Use Case

Validate ADRs against a set of rules.

```rust
use adrscope::application::{ValidateOptions, ValidateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = ValidateUseCase::new(fs);

let options = ValidateOptions::new("docs/decisions")
    .with_strict(true);

let result = use_case.execute(&options)?;
if result.is_valid {
    println!("All ADRs are valid!");
} else {
    println!("Found {} issues", result.total_issues);
}
```

**Types**:
- `ValidateUseCase<F: FileSystem>`
- `ValidateOptions`
- `ValidateResult`

**Methods**:

#### `ValidateOptions::new(input: impl Into<PathBuf>) -> Self`

Create options with input directory.

#### `ValidateOptions::with_strict(self, strict: bool) -> Self`

Enable strict mode (warnings treated as errors). Default: `false`

#### `ValidateOptions::with_pattern(self, pattern: impl Into<String>) -> Self`

Set glob pattern. Default: `**/*.md`

#### `ValidateUseCase::execute(&self, options: &ValidateOptions) -> Result<ValidateResult>`

Execute validation.

**Returns**: `ValidateResult` with `is_valid`, `total_issues`, and detailed `report`

### Stats Use Case

Compute statistics for ADRs.

```rust
use adrscope::application::{StatsOptions, StatsUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = StatsUseCase::new(fs);

let options = StatsOptions::new("docs/decisions");
let result = use_case.execute(&options)?;

println!("Total ADRs: {}", result.stats.total_count);
println!("Accepted: {}", result.stats.by_status.accepted);
```

**Types**:
- `StatsUseCase<F: FileSystem>`
- `StatsOptions`
- `StatsResult`

**Key Fields in `StatsResult::stats`**:
- `total_count: usize` - Total number of ADRs
- `by_status: StatusBreakdown` - Count per status
- `by_category: HashMap<String, usize>` - Count per category
- `recent_activity: Vec<RecentActivity>` - Recent changes

### Wiki Use Case

Generate GitHub Wiki pages from ADRs.

```rust
use adrscope::application::{WikiOptions, WikiUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = WikiUseCase::new(fs);

let options = WikiOptions::new("docs/decisions", "wiki-output");
let result = use_case.execute(&options)?;

println!("Generated {} wiki pages", result.pages_generated);
```

**Types**:
- `WikiUseCase<F: FileSystem>`
- `WikiOptions`
- `WikiResult`

**Generated Pages**:
- `ADR-Index.md` - Main index
- `ADR-By-Status.md` - Grouped by status
- `ADR-By-Category.md` - Grouped by category
- `ADR-Timeline.md` - Chronological view
- `ADR-Statistics.md` - Statistics summary

## Domain Module

Core business logic with zero I/O dependencies.

### ADR Structure

```rust
use adrscope::domain::{Adr, Frontmatter, Status};

pub struct Adr {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub file_path: PathBuf,
}

pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub supersedes: Vec<String>,
    pub superseded_by: Option<String>,
    // ... additional fields
}

pub enum Status {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
}
```

### Facets

Extract search facets from ADRs:

```rust
use adrscope::domain::Facets;

let facets = Facets::from_adrs(&adrs);
println!("Statuses: {:?}", facets.statuses);
println!("Categories: {:?}", facets.categories);
println!("Tags: {:?}", facets.tags);
```

**Key Methods**:
- `Facets::from_adrs(adrs: &[Adr]) -> Self` - Extract facets
- `facets.statuses: Vec<Status>` - Unique statuses
- `facets.categories: Vec<String>` - Unique categories
- `facets.tags: Vec<String>` - Unique tags

### Graph

Build relationship graphs between ADRs:

```rust
use adrscope::domain::Graph;

let graph = Graph::from_adrs(&adrs);
for edge in graph.edges() {
    println!("{} → {}", edge.from, edge.to);
}
```

**Key Methods**:
- `Graph::from_adrs(adrs: &[Adr]) -> Self` - Build graph
- `graph.nodes() -> &[Node]` - Get all nodes
- `graph.edges() -> &[Edge]` - Get all edges
- `graph.find_cycles() -> Vec<Vec<String>>` - Detect cycles

### Statistics

Compute ADR statistics:

```rust
use adrscope::domain::Statistics;

let stats = Statistics::from_adrs(&adrs);
println!("Total: {}", stats.total_count);
println!("Accepted: {}", stats.by_status.accepted);
println!("Proposed: {}", stats.by_status.proposed);
```

**Key Fields**:
- `total_count: usize`
- `by_status: StatusBreakdown`
- `by_category: HashMap<String, usize>`
- `recent_activity: Vec<RecentActivity>`

### Validation

Validate ADRs against rules:

```rust
use adrscope::domain::validation::{ValidationRuleSet, ValidationReport};

let rules = ValidationRuleSet::default();
let report = rules.validate_all(&adrs);

if report.is_valid() {
    println!("All ADRs valid!");
} else {
    for issue in report.issues() {
        println!("{}: {}", issue.severity, issue.message);
    }
}
```

**Available Rules**:
- `RequiredFieldsRule` - Ensures required frontmatter fields
- `StatusTransitionRule` - Validates status transitions
- `UniqueNumbersRule` - Checks for duplicate ADR numbers
- `SupersedesRule` - Validates supersedes relationships
- `FrontmatterFormatRule` - Validates YAML format

**Severity Levels**:
- `Error` - Critical issues
- `Warning` - Non-critical issues
- `Info` - Informational notices

## Infrastructure Module

External concerns: file I/O, parsing, rendering.

### FileSystem Trait

All file I/O goes through this trait for testability:

```rust
use adrscope::infrastructure::fs::{FileSystem, RealFileSystem};
use std::path::Path;

pub trait FileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, contents: &str) -> Result<()>;
    fn glob(&self, pattern: &str) -> Result<Vec<PathBuf>>;
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
#[cfg(test)]
use adrscope::infrastructure::fs::InMemoryFileSystem;

#[test]
fn test_example() {
    let mut fs = InMemoryFileSystem::new();
    fs.add_file("test.md", "# Test ADR");
    
    let content = fs.read_to_string(Path::new("test.md")).unwrap();
    assert_eq!(content, "# Test ADR");
}
```

**Note**: `InMemoryFileSystem` is only available with the `testing` feature or in test builds.

### Parser

Parse ADRs from Markdown files:

```rust
use adrscope::infrastructure::parser::parse_adr;
use std::path::Path;

let content = std::fs::read_to_string("adr.md")?;
let adr = parse_adr(&content, Path::new("adr.md"))?;

println!("Title: {}", adr.frontmatter.title);
println!("Status: {:?}", adr.frontmatter.status);
```

**Key Functions**:
- `parse_adr(content: &str, path: &Path) -> Result<Adr>` - Parse complete ADR
- `extract_frontmatter(content: &str) -> Result<(String, String)>` - Extract YAML
- `parse_frontmatter(yaml: &str) -> Result<Frontmatter>` - Parse YAML to struct

### Renderer

Render ADRs to HTML or Wiki format:

```rust
use adrscope::infrastructure::renderer::{HtmlRenderer, WikiRenderer};

// HTML rendering
let html = HtmlRenderer::render(&adrs, &options)?;
std::fs::write("viewer.html", html)?;

// Wiki rendering
let wiki = WikiRenderer::new();
let pages = wiki.render(&adrs)?;
for (filename, content) in pages {
    std::fs::write(format!("wiki/{}", filename), content)?;
}
```

## Error Module

Unified error handling:

```rust
use adrscope::{Error, Result};

pub enum Error {
    Io(std::io::Error),
    Parse(String),
    Validation(String),
    Render(String),
    NotFound(PathBuf),
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
use adrscope::domain::validation::{ValidationRule, ValidationIssue, Severity};
use adrscope::domain::Adr;

struct MinimumContentLengthRule {
    min_length: usize,
}

impl ValidationRule for MinimumContentLengthRule {
    fn name(&self) -> &'static str {
        "minimum-content-length"
    }
    
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        if adr.content.len() < self.min_length {
            vec![ValidationIssue {
                rule: self.name(),
                severity: Severity::Warning,
                message: format!(
                    "Content length {} is below minimum {}",
                    adr.content.len(),
                    self.min_length
                ),
                file: adr.file_path.clone(),
            }]
        } else {
            vec![]
        }
    }
}
```

### Filtering ADRs

```rust
use adrscope::domain::{Adr, Status};

fn filter_accepted(adrs: Vec<Adr>) -> Vec<Adr> {
    adrs.into_iter()
        .filter(|adr| matches!(adr.frontmatter.status, Status::Accepted))
        .collect()
}

fn filter_by_category(adrs: Vec<Adr>, category: &str) -> Vec<Adr> {
    adrs.into_iter()
        .filter(|adr| {
            adr.frontmatter.category.as_deref() == Some(category)
        })
        .collect()
}
```

### Custom HTML Generation

```rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

fn generate_multiple_viewers() -> adrscope::Result<()> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);
    
    // Generate by status
    for status in ["proposed", "accepted", "deprecated"] {
        let options = GenerateOptions::new("docs/decisions")
            .with_output(format!("{}-adrs.html", status))
            .with_title(format!("{} ADRs", status));
        
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
