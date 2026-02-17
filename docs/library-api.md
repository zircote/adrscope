# Library API Guide

ADRScope can be used as a Rust library to programmatically generate HTML viewers, validate ADRs, generate statistics, and create wiki pages.

## Installation

Add ADRScope to your `Cargo.toml`:

```toml
[dependencies]
adrscope = "0.3"
```

## Core Concepts

ADRScope follows Clean Architecture with four main layers:

- **Application Layer** - Use cases that orchestrate operations
- **Domain Layer** - Core business logic and types
- **Infrastructure Layer** - File I/O, parsing, rendering
- **CLI Layer** - Command-line interface (not needed for library usage)

## Quick Start

````rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

fn main() -> Result<(), adrscope::Error> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);
    
    let options = GenerateOptions::new("docs/decisions")
        .with_output("adr-viewer.html");
    
    let result = use_case.execute(&options)?;
    println!("Generated viewer with {} ADRs", result.adr_count);
    
    Ok(())
}
````

## Use Cases

### Generate HTML Viewer

Create an interactive HTML viewer from your ADRs:

````rust
use adrscope::application::{GenerateOptions, GenerateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;
use adrscope::infrastructure::Theme;

let fs = RealFileSystem::new();
let use_case = GenerateUseCase::new(fs);

let options = GenerateOptions::new("docs/decisions")
    .with_output("adr-viewer.html")
    .with_title("My Architecture Decisions")
    .with_theme(Theme::Dark)
    .with_pattern("**/*.md");

let result = use_case.execute(&options)?;
println!("Generated {} ADRs", result.adr_count);
````

**Options:**

| Method | Default | Description |
|--------|---------|-------------|
| `new(input_dir)` | - | Create options with input directory |
| `with_output(path)` | `adrs.html` | Set output file path |
| `with_title(title)` | `Architecture Decision Records` | Set page title |
| `with_theme(theme)` | `Theme::Auto` | Set color theme |
| `with_pattern(glob)` | `**/*.md` | Set file matching pattern |

**Result:**

````rust
pub struct GenerateResult {
    pub adr_count: usize,
    pub output_path: String,
}
````

### Validate ADRs

Check ADRs for required fields and best practices:

````rust
use adrscope::application::{ValidateOptions, ValidateUseCase};
use adrscope::infrastructure::fs::RealFileSystem;
use adrscope::domain::Severity;

let fs = RealFileSystem::new();
let use_case = ValidateUseCase::new(fs);

let options = ValidateOptions::new("docs/decisions")
    .with_strict(true)  // Fail on warnings
    .with_pattern("**/*.md");

let result = use_case.execute(&options)?;

if result.has_errors() {
    eprintln!("Validation failed!");
    for issue in result.issues() {
        if matches!(issue.severity, Severity::Error) {
            eprintln!("ERROR: {} - {}", issue.file, issue.message);
        }
    }
}
````

**Options:**

| Method | Default | Description |
|--------|---------|-------------|
| `new(input_dir)` | - | Create options with input directory |
| `with_pattern(glob)` | `**/*.md` | Set file matching pattern |
| `with_strict(bool)` | `false` | Fail on warnings |

**Result:**

````rust
pub struct ValidateResult {
    pub report: ValidationReport,
    pub adr_count: usize,
}

impl ValidateResult {
    pub fn has_errors(&self) -> bool;
    pub fn has_warnings(&self) -> bool;
    pub fn issues(&self) -> &[ValidationIssue];
}
````

### Generate Statistics

Analyze your ADR collection:

````rust
use adrscope::application::{StatsOptions, StatsUseCase, StatsFormat};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = StatsUseCase::new(fs);

let options = StatsOptions::new("docs/decisions")
    .with_format(StatsFormat::Json);

let result = use_case.execute(&options)?;
println!("{}", result.formatted_output);
````

**Options:**

| Method | Default | Description |
|--------|---------|-------------|
| `new(input_dir)` | - | Create options with input directory |
| `with_pattern(glob)` | `**/*.md` | Set file matching pattern |
| `with_format(format)` | `StatsFormat::Text` | Output format |

**Formats:**

- `StatsFormat::Text` - Human-readable text
- `StatsFormat::Json` - Machine-readable JSON
- `StatsFormat::Markdown` - Documentation-ready markdown

**Result:**

````rust
pub struct StatsResult {
    pub statistics: AdrStatistics,
    pub formatted_output: String,
}
````

### Generate Wiki Pages

Create GitHub Wiki-compatible markdown pages:

````rust
use adrscope::application::{WikiOptions, WikiUseCase};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let use_case = WikiUseCase::new(fs);

let options = WikiOptions::new("docs/decisions")
    .with_output_dir("wiki/")
    .with_pages_url("https://example.github.io/adr-viewer.html")
    .with_pattern("**/*.md");

let result = use_case.execute(&options)?;
println!("Generated {} wiki pages", result.page_count);
````

**Options:**

| Method | Default | Description |
|--------|---------|-------------|
| `new(input_dir)` | - | Create options with input directory |
| `with_output_dir(dir)` | `wiki/` | Set output directory |
| `with_pages_url(url)` | `None` | Link to GitHub Pages viewer |
| `with_pattern(glob)` | `**/*.md` | Set file matching pattern |

**Generated Pages:**

- `ADR-Index.md` - Main index
- `ADR-By-Status.md` - Grouped by status
- `ADR-By-Category.md` - Grouped by category
- `ADR-Timeline.md` - Chronological view
- `ADR-Statistics.md` - Summary statistics

## Domain Types

### Adr

Represents a parsed Architecture Decision Record:

````rust
use adrscope::domain::Adr;

// Access ADR properties
let id = adr.id();
let frontmatter = adr.frontmatter();
let body_html = adr.body_html();
let body_markdown = adr.body_markdown();
let filename = adr.filename();
let source_path = adr.source_path();
````

### Frontmatter

YAML metadata from the ADR:

````rust
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub created: Option<String>,
    pub author: Option<String>,
    pub project: Option<String>,
    pub technologies: Vec<String>,
    pub audience: Vec<String>,
    pub related: Vec<String>,
}
````

### Status

ADR lifecycle status:

````rust
pub enum Status {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
}

// Parse from string
let status = Status::from_str("accepted")?;

// Convert to string
let s = status.to_string();  // "accepted"
````

### ValidationReport

Validation results:

````rust
pub struct ValidationReport {
    // Access validation issues
    pub fn issues(&self) -> &[ValidationIssue];
    pub fn has_errors(&self) -> bool;
    pub fn has_warnings(&self) -> bool;
}

pub struct ValidationIssue {
    pub file: String,
    pub severity: Severity,
    pub message: String,
    pub rule: String,
}

pub enum Severity {
    Error,
    Warning,
}
````

### AdrStatistics

Statistical analysis:

````rust
pub struct AdrStatistics {
    pub total_count: usize,
    pub by_status: HashMap<Status, usize>,
    pub by_category: HashMap<String, usize>,
    pub top_tags: Vec<(String, usize)>,
    pub top_authors: Vec<(String, usize)>,
    pub date_range: Option<(String, String)>,
}
````

## FileSystem Trait

ADRScope uses a trait-based abstraction for file I/O, enabling testability:

````rust
pub trait FileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
}
````

### RealFileSystem

Production implementation:

````rust
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
````

### InMemoryFileSystem (Testing)

In-memory implementation for testing:

````rust
#[cfg(test)]
use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;

#[test]
fn test_generate() {
    let mut fs = InMemoryFileSystem::new();
    fs.add_file("docs/decisions/adr-001.md", "---\ntitle: Test\nstatus: accepted\n---\n\n## Context\n\nTest ADR");
    
    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new("docs/decisions");
    
    let result = use_case.execute(&options).unwrap();
    assert_eq!(result.adr_count, 1);
}
````

**Note:** `InMemoryFileSystem` requires the `testing` feature or `cfg(test)`.

## Error Handling

All operations return `Result<T, adrscope::Error>`:

````rust
use adrscope::Error;

match use_case.execute(&options) {
    Ok(result) => println!("Success!"),
    Err(Error::Io(e)) => eprintln!("I/O error: {}", e),
    Err(Error::Parse(e)) => eprintln!("Parse error: {}", e),
    Err(Error::Validation(e)) => eprintln!("Validation error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
````

**Error Variants:**

- `Error::Io` - File system errors
- `Error::Parse` - Markdown or YAML parsing errors
- `Error::Validation` - ADR validation failures
- `Error::Render` - HTML rendering errors
- `Error::NoAdrsFound` - No matching ADR files

## Advanced Usage

### Custom Validation Rules

Implement custom validation logic:

````rust
use adrscope::domain::{ValidationRule, ValidationReport, Adr};

struct CustomRule;

impl ValidationRule for CustomRule {
    fn name(&self) -> &str {
        "custom-rule"
    }
    
    fn description(&self) -> &str {
        "Custom validation logic"
    }
    
    fn validate(&self, adr: &Adr, report: &mut ValidationReport) {
        // Custom validation logic
        if adr.frontmatter().title.len() < 10 {
            report.add_warning(
                adr.filename(),
                "Title should be at least 10 characters"
            );
        }
    }
}

// Use custom rule
use adrscope::domain::Validator;

let mut validator = Validator::new();
validator.add_rule(Box::new(CustomRule));

let report = validator.validate(&adrs);
````

### Direct Parser Usage

Parse ADRs without use cases:

````rust
use adrscope::infrastructure::{AdrParser, DefaultAdrParser};
use adrscope::infrastructure::fs::RealFileSystem;

let fs = RealFileSystem::new();
let parser = DefaultAdrParser::new();

let content = fs.read_to_string("docs/decisions/adr-001.md".as_ref())?;
let adr = parser.parse(&content, "adr-001.md")?;

println!("Title: {}", adr.frontmatter().title);
println!("Status: {}", adr.frontmatter().status);
````

### Direct Renderer Usage

Generate HTML without use cases:

````rust
use adrscope::infrastructure::{HtmlRenderer, RenderConfig, Theme};

let renderer = HtmlRenderer::new();
let config = RenderConfig {
    title: "My ADRs".to_string(),
    theme: Theme::Dark,
};

let html = renderer.render(&adrs, &config)?;
````

## Testing

ADRScope is designed for testability with dependency injection:

````rust
#[cfg(test)]
mod tests {
    use super::*;
    use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;
    
    #[test]
    fn test_generate_viewer() {
        // Arrange
        let mut fs = InMemoryFileSystem::new();
        fs.add_file(
            "docs/decisions/adr-001.md",
            r#"---
title: Test Decision
status: accepted
---

## Context
Test context
"#
        );
        
        // Act
        let use_case = GenerateUseCase::new(fs);
        let options = GenerateOptions::new("docs/decisions");
        let result = use_case.execute(&options).unwrap();
        
        // Assert
        assert_eq!(result.adr_count, 1);
    }
}
````

## Feature Flags

ADRScope currently has one optional feature:

- `testing` - Enables `InMemoryFileSystem` for testing (automatically enabled with `cfg(test)`)

## Examples

See the [examples directory](https://github.com/zircote/adrscope/tree/main/examples) for complete working examples:

- `basic_usage.rs` - Simple HTML generation
- `validation.rs` - Validating ADRs programmatically
- `custom_rules.rs` - Custom validation rules
- `statistics.rs` - Generating statistics
- `wiki.rs` - Wiki page generation

## API Documentation

For complete API documentation, visit [docs.rs/adrscope](https://docs.rs/adrscope).

## See Also

- [User Guide](user-guide.md) - Command-line usage
- [Configuration Reference](configuration.md) - CLI options
- [Getting Started](getting-started.md) - Quick start tutorial
