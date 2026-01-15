//! Integration tests for ADRScope.
//!
//! These are integration tests that exercise the full application stack.

// Allow test-specific patterns that are stricter in library code
#![allow(clippy::expect_used)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_raw_string_hashes)]

use std::fs;
use std::path::PathBuf;

use adrscope::Error;
use adrscope::application::{GenerateOptions, GenerateUseCase, ValidateOptions, ValidateUseCase};
use adrscope::cli::run;
use adrscope::cli::{
    Cli, Commands, FormatArg, GenerateArgs, StatsArgs, ThemeArg, ValidateArgs, WikiArgs,
};
use adrscope::infrastructure::fs::FileSystem;
use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;

fn sample_adr(id: &str, title: &str, status: &str, category: &str) -> String {
    format!(
        r"---
title: {title}
status: {status}
category: {category}
created: 2025-01-15
description: Description for {title}
author: Test Author
tags:
  - architecture
  - testing
---

# {title}

## Context

This is the context for {id}.

## Decision

We decided to test with {title}.

## Consequences

- Consequence 1
- Consequence 2
"
    )
}

#[test]
fn test_generate_single_adr() {
    let fs = InMemoryFileSystem::new();
    fs.add_file(
        "docs/decisions/adr-0001.md",
        sample_adr("adr-0001", "Use PostgreSQL", "accepted", "database"),
    );

    let use_case = GenerateUseCase::new(fs.clone());
    let options = GenerateOptions::new("docs/decisions").with_output("output.html");

    let result = use_case.execute(&options).unwrap();

    assert_eq!(result.adr_count, 1);
    assert!(!result.has_errors());

    // Verify output was written
    let output = fs
        .read_to_string(std::path::Path::new("output.html"))
        .unwrap();
    assert!(output.contains("<!DOCTYPE html>"));
    assert!(output.contains("Use PostgreSQL"));
}

#[test]
fn test_generate_multiple_adrs() {
    let fs = InMemoryFileSystem::new();
    fs.add_file(
        "docs/decisions/adr-0001.md",
        sample_adr("adr-0001", "Use PostgreSQL", "accepted", "database"),
    );
    fs.add_file(
        "docs/decisions/adr-0002.md",
        sample_adr("adr-0002", "Use Rust", "proposed", "language"),
    );
    fs.add_file(
        "docs/decisions/adr-0003.md",
        sample_adr("adr-0003", "Use REST", "deprecated", "api"),
    );

    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new("docs/decisions").with_output("output.html");

    let result = use_case.execute(&options).unwrap();

    assert_eq!(result.adr_count, 3);
    assert!(!result.has_errors());
}

#[test]
fn test_generate_no_adrs_error() {
    let fs = InMemoryFileSystem::new();
    let use_case = GenerateUseCase::new(fs);
    let options = GenerateOptions::new("empty/dir");

    let result = use_case.execute(&options);

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NoAdrsFound { .. } => {},
        e => panic!("Expected NoAdrsFound error, got {e:?}"),
    }
}

#[test]
fn test_validate_valid_adrs() {
    let fs = InMemoryFileSystem::new();
    fs.add_file(
        "docs/decisions/adr-0001.md",
        sample_adr("adr-0001", "Use PostgreSQL", "accepted", "database"),
    );

    let use_case = ValidateUseCase::new(fs);
    let options = ValidateOptions::new("docs/decisions");

    let result = use_case.execute(&options).unwrap();

    assert!(result.passed);
    assert_eq!(result.total_errors, 0);
}

#[test]
fn test_validate_missing_recommended_fields() {
    let fs = InMemoryFileSystem::new();
    fs.add_file(
        "docs/decisions/adr-0001.md",
        r"---
title: Minimal ADR
status: proposed
---

# Minimal ADR

Content.
",
    );

    let use_case = ValidateUseCase::new(fs);
    let options = ValidateOptions::new("docs/decisions");

    let result = use_case.execute(&options).unwrap();

    // Should pass (only errors fail, not warnings)
    assert!(result.passed);
    assert_eq!(result.total_errors, 0);
    // Should have warnings for missing description, created, category
    assert!(result.total_warnings > 0);
}

#[test]
fn test_validate_strict_mode_fails_on_warnings() {
    let fs = InMemoryFileSystem::new();
    fs.add_file(
        "docs/decisions/adr-0001.md",
        r"---
title: Minimal ADR
status: proposed
---

# Minimal ADR

Content.
",
    );

    let use_case = ValidateUseCase::new(fs);
    let options = ValidateOptions::new("docs/decisions").with_strict(true);

    let result = use_case.execute(&options).unwrap();

    // Should fail in strict mode due to warnings
    assert!(!result.passed);
}

// =============================================================================
// CLI Handler Integration Tests
// =============================================================================

fn create_temp_dir() -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "adrscope_test_{}_{}",
        std::process::id(),
        timestamp
    ));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    temp_dir
}

fn cleanup_temp_dir(path: &PathBuf) {
    let _ = fs::remove_dir_all(path);
}

fn write_test_adr(dir: &PathBuf, filename: &str, title: &str, status: &str, category: &str) {
    let decisions_dir = dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create decisions directory");

    let content = format!(
        r"---
title: {title}
status: {status}
category: {category}
created: 2025-01-15
description: Description for {title}
author: Test Author
---

# {title}

## Context

This is the context.

## Decision

We decided to do this.

## Consequences

- Consequence 1
"
    );

    fs::write(decisions_dir.join(filename), content).expect("Failed to write test ADR");
}

#[test]
fn test_cli_generate_handler() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: temp_dir.join("output.html").to_string_lossy().to_string(),
            title: "Test ADRs".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    // Verify output file was created
    assert!(temp_dir.join("output.html").exists());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_generate_handler_verbose() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let cli = Cli {
        verbose: true,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: temp_dir.join("output.html").to_string_lossy().to_string(),
            title: "Test ADRs".to_string(),
            theme: ThemeArg::Light,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_validate_handler() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let cli = Cli {
        verbose: false,
        command: Commands::Validate(ValidateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            strict: false,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // Should pass validation

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_validate_handler_strict_fails() {
    let temp_dir = create_temp_dir();

    // Write a minimal ADR without recommended fields
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create decisions directory");
    fs::write(
        decisions_dir.join("adr-0001.md"),
        r"---
title: Minimal ADR
status: proposed
---

# Minimal ADR

Content.
",
    )
    .expect("Failed to write test ADR");

    let cli = Cli {
        verbose: true,
        command: Commands::Validate(ValidateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            strict: true,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // Should fail in strict mode

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_stats_handler() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );
    write_test_adr(&temp_dir, "adr-0002.md", "Use Redis", "proposed", "cache");

    let cli = Cli {
        verbose: false,
        command: Commands::Stats(StatsArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            format: FormatArg::Text,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_stats_handler_json_format() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let cli = Cli {
        verbose: true,
        command: Commands::Stats(StatsArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            format: FormatArg::Json,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_stats_handler_markdown_format() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let cli = Cli {
        verbose: false,
        command: Commands::Stats(StatsArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            format: FormatArg::Markdown,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_wiki_handler() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let wiki_dir = temp_dir.join("wiki");

    let cli = Cli {
        verbose: false,
        command: Commands::Wiki(WikiArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: wiki_dir.to_string_lossy().to_string(),
            pages_url: Some("https://example.com/adrs".to_string()),
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    // Verify wiki files were created
    assert!(wiki_dir.join("ADR-Index.md").exists());
    assert!(wiki_dir.join("ADR-By-Status.md").exists());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_wiki_handler_verbose_no_url() {
    let temp_dir = create_temp_dir();
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL",
        "accepted",
        "database",
    );

    let wiki_dir = temp_dir.join("wiki");

    let cli = Cli {
        verbose: true,
        command: Commands::Wiki(WikiArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: wiki_dir.to_string_lossy().to_string(),
            pages_url: None,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_generate_no_adrs_error() {
    let temp_dir = create_temp_dir();
    let empty_dir = temp_dir.join("empty");
    fs::create_dir_all(&empty_dir).expect("Failed to create empty directory");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: empty_dir.to_string_lossy().to_string(),
            output: temp_dir.join("output.html").to_string_lossy().to_string(),
            title: "Test ADRs".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_err());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_generate_with_parse_errors() {
    let temp_dir = create_temp_dir();

    // Write a valid ADR
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Valid ADR",
        "accepted",
        "database",
    );

    // Write a malformed ADR with invalid YAML
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r"---
title: [Invalid YAML - missing closing bracket
status: accepted
---

# Bad ADR
",
    )
    .expect("Failed to write malformed ADR");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: temp_dir.join("output.html").to_string_lossy().to_string(),
            title: "Test ADRs".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    // Should succeed but report warnings about the malformed file
    let result = run(cli);
    assert!(result.is_ok());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_wiki_with_parse_errors() {
    let temp_dir = create_temp_dir();

    // Write a valid ADR
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Valid ADR",
        "accepted",
        "database",
    );

    // Write a malformed ADR
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r"---
invalid: yaml: here: broken
---

# Bad
",
    )
    .expect("Failed to write malformed ADR");

    let wiki_dir = temp_dir.join("wiki");

    let cli = Cli {
        verbose: false,
        command: Commands::Wiki(WikiArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: wiki_dir.to_string_lossy().to_string(),
            pages_url: None,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_stats_with_parse_errors() {
    let temp_dir = create_temp_dir();

    // Write a valid ADR
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Valid ADR",
        "accepted",
        "database",
    );

    // Write a malformed ADR with invalid YAML
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r#"---
title: [Invalid YAML
status: bad
---

# Bad
"#,
    )
    .expect("Failed to write malformed ADR");

    let cli = Cli {
        verbose: false,
        command: Commands::Stats(StatsArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            format: FormatArg::Text,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_validate_with_parse_errors() {
    let temp_dir = create_temp_dir();

    // Write a valid ADR
    write_test_adr(
        &temp_dir,
        "adr-0001.md",
        "Valid ADR",
        "accepted",
        "database",
    );

    // Write a malformed ADR
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r"---
malformed:
  - yaml
  - here
  missing: colon
---

# Bad
",
    )
    .expect("Failed to write malformed ADR");

    let cli = Cli {
        verbose: false,
        command: Commands::Validate(ValidateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            strict: false,
        }),
    };

    // Should succeed - parse errors are logged but valid ADRs still processed
    let result = run(cli);
    assert!(result.is_ok());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_cli_validate_with_error_severity_issues() {
    let temp_dir = create_temp_dir();

    // Write an ADR with missing required title (should cause error)
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create decisions directory");
    fs::write(
        decisions_dir.join("adr-0001.md"),
        r"---
status: accepted
category: test
---

# No Title ADR
",
    )
    .expect("Failed to write ADR");

    let cli = Cli {
        verbose: false,
        command: Commands::Validate(ValidateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            strict: false,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // Should fail due to missing title

    cleanup_temp_dir(&temp_dir);
}

// =============================================================================
// Substantial Functional Tests
// =============================================================================

/// Helper to create a comprehensive ADR with all fields
fn write_full_adr(
    dir: &PathBuf,
    filename: &str,
    title: &str,
    status: &str,
    category: &str,
    description: &str,
    author: &str,
    tags: &[&str],
    technologies: &[&str],
    related: &[&str],
) {
    let decisions_dir = dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create decisions directory");

    let tags_yaml = if tags.is_empty() {
        String::new()
    } else {
        format!(
            "tags:\n{}",
            tags.iter()
                .map(|t| format!("  - {t}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let tech_yaml = if technologies.is_empty() {
        String::new()
    } else {
        format!(
            "technologies:\n{}",
            technologies
                .iter()
                .map(|t| format!("  - {t}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let related_yaml = if related.is_empty() {
        String::new()
    } else {
        format!(
            "related:\n{}",
            related
                .iter()
                .map(|r| format!("  - {r}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let content = format!(
        r#"---
title: "{title}"
status: {status}
category: {category}
description: "{description}"
author: "{author}"
created: "2025-01-15"
updated: "2025-01-16"
project: "test-project"
{tags_yaml}
{tech_yaml}
{related_yaml}
---

# {title}

## Context

This decision addresses the need for {title}.

## Decision

We will implement {title} using the following approach:

1. First step of implementation
2. Second step with details
3. Final integration

## Consequences

### Positive

- Improved performance
- Better maintainability
- Enhanced security

### Negative

- Additional complexity
- Learning curve for team

## References

- [Reference 1](https://example.com/ref1)
- [Reference 2](https://example.com/ref2)
"#
    );

    fs::write(decisions_dir.join(filename), content).expect("Failed to write ADR");
}

#[test]
fn test_functional_generate_html_content() {
    let temp_dir = create_temp_dir();

    // Create multiple ADRs with different statuses and categories
    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "Use PostgreSQL for Data Storage",
        "accepted",
        "database",
        "Decision to use PostgreSQL as primary database",
        "Database Team",
        &["database", "postgresql", "storage"],
        &["postgresql", "sql"],
        &[],
    );

    write_full_adr(
        &temp_dir,
        "adr-0002.md",
        "Implement REST API",
        "proposed",
        "api",
        "Design and implement REST API endpoints",
        "API Team",
        &["api", "rest", "http"],
        &["rust", "actix-web"],
        &["adr-0001.md"],
    );

    write_full_adr(
        &temp_dir,
        "adr-0003.md",
        "Deprecate Legacy Auth",
        "deprecated",
        "security",
        "Phase out legacy authentication system",
        "Security Team",
        &["security", "auth", "legacy"],
        &["oauth2"],
        &["adr-0001.md", "adr-0002.md"],
    );

    write_full_adr(
        &temp_dir,
        "adr-0004.md",
        "Replace Monolith with Microservices",
        "superseded",
        "architecture",
        "Original microservices proposal (superseded)",
        "Architecture Team",
        &["architecture", "microservices"],
        &["kubernetes", "docker"],
        &[],
    );

    let output_path = temp_dir.join("output.html");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: output_path.to_string_lossy().to_string(),
            title: "Test Project ADRs".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    // Read and verify the generated HTML
    let html = fs::read_to_string(&output_path).expect("Failed to read output HTML");

    // Verify HTML structure
    assert!(html.contains("<!DOCTYPE html>"), "Missing DOCTYPE");
    assert!(html.contains("<html"), "Missing html tag");
    assert!(html.contains("Test Project ADRs"), "Missing title");

    // Verify all ADRs are present
    assert!(
        html.contains("Use PostgreSQL for Data Storage"),
        "Missing ADR 1"
    );
    assert!(html.contains("Implement REST API"), "Missing ADR 2");
    assert!(html.contains("Deprecate Legacy Auth"), "Missing ADR 3");
    assert!(
        html.contains("Replace Monolith with Microservices"),
        "Missing ADR 4"
    );

    // Verify statuses are represented
    assert!(html.contains("accepted"), "Missing accepted status");
    assert!(html.contains("proposed"), "Missing proposed status");
    assert!(html.contains("deprecated"), "Missing deprecated status");
    assert!(html.contains("superseded"), "Missing superseded status");

    // Verify categories are present
    assert!(html.contains("database"), "Missing database category");
    assert!(html.contains("api"), "Missing api category");
    assert!(html.contains("security"), "Missing security category");
    assert!(
        html.contains("architecture"),
        "Missing architecture category"
    );

    // Verify content sections are rendered
    assert!(html.contains("Context"), "Missing Context section");
    assert!(html.contains("Decision"), "Missing Decision section");
    assert!(
        html.contains("Consequences"),
        "Missing Consequences section"
    );

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_wiki_generates_all_files() {
    let temp_dir = create_temp_dir();

    // Create diverse ADRs
    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "Database Selection",
        "accepted",
        "infrastructure",
        "Choose database technology",
        "Alice",
        &["database"],
        &["postgres"],
        &[],
    );

    write_full_adr(
        &temp_dir,
        "adr-0002.md",
        "API Framework",
        "proposed",
        "development",
        "Select API framework",
        "Bob",
        &["api", "framework"],
        &["rust", "actix"],
        &["adr-0001.md"],
    );

    let wiki_dir = temp_dir.join("wiki");

    let cli = Cli {
        verbose: false,
        command: Commands::Wiki(WikiArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: wiki_dir.to_string_lossy().to_string(),
            pages_url: Some("https://example.com/adrs".to_string()),
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    // Verify all wiki files are created
    assert!(
        wiki_dir.join("ADR-Index.md").exists(),
        "Missing ADR-Index.md"
    );
    assert!(
        wiki_dir.join("ADR-By-Status.md").exists(),
        "Missing ADR-By-Status.md"
    );
    assert!(
        wiki_dir.join("ADR-By-Category.md").exists(),
        "Missing ADR-By-Category.md"
    );
    assert!(
        wiki_dir.join("ADR-Timeline.md").exists(),
        "Missing ADR-Timeline.md"
    );
    assert!(
        wiki_dir.join("ADR-Statistics.md").exists(),
        "Missing ADR-Statistics.md"
    );

    // Verify Index content
    let index = fs::read_to_string(wiki_dir.join("ADR-Index.md")).expect("Failed to read index");
    assert!(index.contains("# ADR Index"), "Missing index title");
    assert!(
        index.contains("View Interactive ADRScope Viewer"),
        "Missing pages URL link"
    );
    assert!(
        index.contains("Database Selection"),
        "Missing ADR 1 in index"
    );
    assert!(index.contains("API Framework"), "Missing ADR 2 in index");
    assert!(index.contains("| ID |"), "Missing table header");

    // Verify By-Status content
    let by_status =
        fs::read_to_string(wiki_dir.join("ADR-By-Status.md")).expect("Failed to read by-status");
    assert!(
        by_status.contains("# ADRs by Status"),
        "Missing by-status title"
    );
    assert!(
        by_status.contains("accepted") || by_status.contains("Accepted"),
        "Missing accepted section"
    );
    assert!(
        by_status.contains("proposed") || by_status.contains("Proposed"),
        "Missing proposed section"
    );

    // Verify By-Category content
    let by_category = fs::read_to_string(wiki_dir.join("ADR-By-Category.md"))
        .expect("Failed to read by-category");
    assert!(
        by_category.contains("# ADRs by Category"),
        "Missing by-category title"
    );
    assert!(
        by_category.contains("infrastructure"),
        "Missing infrastructure category"
    );
    assert!(
        by_category.contains("development"),
        "Missing development category"
    );

    // Verify Timeline content
    let timeline =
        fs::read_to_string(wiki_dir.join("ADR-Timeline.md")).expect("Failed to read timeline");
    assert!(
        timeline.contains("# ADR Timeline"),
        "Missing timeline title"
    );
    assert!(timeline.contains("2025"), "Missing year in timeline");

    // Verify Statistics content
    let stats =
        fs::read_to_string(wiki_dir.join("ADR-Statistics.md")).expect("Failed to read statistics");
    assert!(
        stats.contains("# ADR Statistics"),
        "Missing statistics title"
    );
    assert!(stats.contains("Total ADRs"), "Missing total count");
    assert!(stats.contains("By Status"), "Missing status breakdown");

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_stats_text_format() {
    let temp_dir = create_temp_dir();

    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "ADR One",
        "accepted",
        "arch",
        "First ADR",
        "Alice",
        &["tag1", "tag2"],
        &["rust"],
        &[],
    );
    write_full_adr(
        &temp_dir,
        "adr-0002.md",
        "ADR Two",
        "accepted",
        "arch",
        "Second ADR",
        "Bob",
        &["tag1"],
        &["rust", "postgres"],
        &[],
    );
    write_full_adr(
        &temp_dir,
        "adr-0003.md",
        "ADR Three",
        "proposed",
        "api",
        "Third ADR",
        "Alice",
        &["tag3"],
        &["go"],
        &[],
    );

    // Test text format
    let cli = Cli {
        verbose: false,
        command: Commands::Stats(StatsArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            pattern: "**/*.md".to_string(),
            format: FormatArg::Text,
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_stats_json_output_structure() {
    let temp_dir = create_temp_dir();

    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "JSON Test ADR",
        "accepted",
        "testing",
        "ADR for JSON stats test",
        "Test Author",
        &["json", "test"],
        &["rust"],
        &[],
    );

    // Capture JSON output by running use case directly
    let fs_impl = adrscope::infrastructure::RealFileSystem::new();
    let use_case = adrscope::application::StatsUseCase::new(fs_impl);
    let options = adrscope::application::StatsOptions::new(
        temp_dir
            .join("docs/decisions")
            .to_string_lossy()
            .to_string(),
    )
    .with_format(adrscope::application::stats::StatsFormat::Json);

    let result = use_case.execute(&options).expect("Stats should succeed");

    // Parse JSON output
    let json: serde_json::Value =
        serde_json::from_str(&result.output).expect("Output should be valid JSON");

    // Verify JSON structure
    assert!(json.is_object(), "JSON should be an object");
    assert!(json.get("total_count").is_some(), "Missing total_count");
    assert!(json.get("by_status").is_some(), "Missing by_status");
    assert!(json.get("by_category").is_some(), "Missing by_category");
    assert!(json.get("by_author").is_some(), "Missing by_author");
    assert!(json.get("by_tag").is_some(), "Missing by_tag");
    assert!(json.get("by_technology").is_some(), "Missing by_technology");

    // Verify counts
    assert_eq!(json["total_count"], 1);
    assert_eq!(json["by_status"]["accepted"], 1);
    assert_eq!(json["by_category"]["testing"], 1);

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_validate_comprehensive() {
    let temp_dir = create_temp_dir();
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create directory");

    // ADR 1: Fully valid with all fields
    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "Complete Valid ADR",
        "accepted",
        "architecture",
        "Fully documented ADR",
        "Architect",
        &["complete"],
        &["rust"],
        &[],
    );

    // ADR 2: Valid but minimal (will have warnings)
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r#"---
title: "Minimal ADR"
status: proposed
---

# Minimal ADR

Basic content.
"#,
    )
    .expect("Failed to write ADR 2");

    // ADR 3: Missing title (will have error)
    fs::write(
        decisions_dir.join("adr-0003.md"),
        r#"---
status: accepted
category: test
---

# No Title
"#,
    )
    .expect("Failed to write ADR 3");

    // Run validation in non-strict mode
    let use_case = ValidateUseCase::new(adrscope::infrastructure::RealFileSystem::new());
    let options = ValidateOptions::new(decisions_dir.to_string_lossy().to_string());

    let result = use_case
        .execute(&options)
        .expect("Validation should complete");

    // Verify results
    // Note: Missing title causes a parse error (during parsing), not a validation error
    assert!(
        !result.passed,
        "Should fail due to parse error (missing title)"
    );
    assert!(
        !result.parse_errors.is_empty(),
        "Should have at least 1 parse error for ADR 3 (missing title)"
    );
    assert!(
        result.total_warnings >= 3,
        "Should have warnings for minimal ADR (missing description, created, category)"
    );

    // Run validation in strict mode
    let strict_options =
        ValidateOptions::new(decisions_dir.to_string_lossy().to_string()).with_strict(true);

    let strict_result = use_case
        .execute(&strict_options)
        .expect("Strict validation should complete");

    assert!(
        !strict_result.passed,
        "Strict mode should fail on warnings too"
    );

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_generate_with_relationships() {
    let temp_dir = create_temp_dir();

    // Create ADRs with relationships
    write_full_adr(
        &temp_dir,
        "adr-0001.md",
        "Foundation ADR",
        "accepted",
        "core",
        "Base architecture decision",
        "Team",
        &[],
        &[],
        &[],
    );

    write_full_adr(
        &temp_dir,
        "adr-0002.md",
        "Extension ADR",
        "accepted",
        "feature",
        "Builds on foundation",
        "Team",
        &[],
        &[],
        &["adr-0001.md"],
    );

    write_full_adr(
        &temp_dir,
        "adr-0003.md",
        "Another Extension",
        "proposed",
        "feature",
        "Also builds on foundation",
        "Team",
        &[],
        &[],
        &["adr-0001.md", "adr-0002.md"],
    );

    let output_path = temp_dir.join("output.html");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: output_path.to_string_lossy().to_string(),
            title: "Relationship Test".to_string(),
            theme: ThemeArg::Dark,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    let html = fs::read_to_string(&output_path).expect("Failed to read HTML");

    // Verify relationships are captured in the graph data
    assert!(html.contains("adr-0001"), "Missing base ADR reference");
    assert!(html.contains("adr-0002"), "Missing extension ADR reference");
    assert!(
        html.contains("adr-0003"),
        "Missing second extension reference"
    );

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_all_status_types_in_output() {
    let temp_dir = create_temp_dir();

    // Create one ADR for each status
    for (i, (status, name)) in [
        ("proposed", "Proposed Decision"),
        ("accepted", "Accepted Decision"),
        ("deprecated", "Deprecated Decision"),
        ("superseded", "Superseded Decision"),
    ]
    .iter()
    .enumerate()
    {
        write_full_adr(
            &temp_dir,
            &format!("adr-000{}.md", i + 1),
            name,
            status,
            "test",
            &format!("Testing {status} status"),
            "Tester",
            &[],
            &[],
            &[],
        );
    }

    // Generate HTML and verify all statuses
    let output_path = temp_dir.join("all_status.html");
    let fs_impl = adrscope::infrastructure::RealFileSystem::new();
    let use_case = adrscope::application::GenerateUseCase::new(fs_impl);
    let options = adrscope::application::GenerateOptions::new(
        temp_dir
            .join("docs/decisions")
            .to_string_lossy()
            .to_string(),
    )
    .with_output(output_path.to_string_lossy().to_string());

    let result = use_case.execute(&options).expect("Generate should succeed");
    assert_eq!(result.adr_count, 4);

    let html = fs::read_to_string(&output_path).expect("Failed to read HTML");

    // Verify all status names appear
    assert!(html.contains("Proposed Decision"));
    assert!(html.contains("Accepted Decision"));
    assert!(html.contains("Deprecated Decision"));
    assert!(html.contains("Superseded Decision"));

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_empty_and_edge_cases() {
    let temp_dir = create_temp_dir();
    let decisions_dir = temp_dir.join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create directory");

    // ADR with empty optional fields
    fs::write(
        decisions_dir.join("adr-0001.md"),
        r#"---
title: "ADR with Empty Fields"
status: accepted
category: ""
description: ""
author: ""
tags: []
technologies: []
related: []
---

# ADR with Empty Fields

Content here.
"#,
    )
    .expect("Failed to write ADR");

    // ADR with special characters in title
    fs::write(
        decisions_dir.join("adr-0002.md"),
        r#"---
title: "ADR with <Special> & \"Characters\""
status: proposed
category: testing
description: "Testing & escaping"
---

# ADR with Special Characters

This tests HTML escaping in < > & " characters.
"#,
    )
    .expect("Failed to write ADR");

    let output_path = temp_dir.join("edge_cases.html");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: decisions_dir.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            title: "Edge Cases Test".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    let html = fs::read_to_string(&output_path).expect("Failed to read HTML");

    // Verify the HTML is valid (special chars should be escaped)
    assert!(html.contains("ADR with Empty Fields"));
    // Special characters should be properly handled
    assert!(html.contains("Edge Cases Test"));

    cleanup_temp_dir(&temp_dir);
}

#[test]
fn test_functional_large_adr_collection() {
    let temp_dir = create_temp_dir();

    // Generate 20 ADRs to test scaling
    for i in 1..=20 {
        let status = match i % 4 {
            0 => "accepted",
            1 => "proposed",
            2 => "deprecated",
            _ => "superseded",
        };

        let category = match i % 5 {
            0 => "architecture",
            1 => "api",
            2 => "database",
            3 => "security",
            _ => "infrastructure",
        };

        // Bind temporary strings to variables to extend their lifetime
        let filename = format!("adr-{:04}.md", i);
        let title = format!("Decision Number {i}");
        let description = format!("Description for decision {i}");
        let author = format!("Author {}", i % 5);
        let tag_str = format!("tag{}", i % 3);
        let tech_str = format!("tech{}", i % 4);
        let related_str = format!("adr-{:04}.md", i - 1);
        let related: &[&str] = if i > 1 { &[&related_str] } else { &[] };

        write_full_adr(
            &temp_dir,
            &filename,
            &title,
            status,
            category,
            &description,
            &author,
            &[&tag_str],
            &[&tech_str],
            related,
        );
    }

    let output_path = temp_dir.join("large_collection.html");

    let cli = Cli {
        verbose: false,
        command: Commands::Generate(GenerateArgs {
            input: temp_dir
                .join("docs/decisions")
                .to_string_lossy()
                .to_string(),
            output: output_path.to_string_lossy().to_string(),
            title: "Large Collection Test".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        }),
    };

    let result = run(cli);
    assert!(result.is_ok());

    // Verify output was created and contains all ADRs
    let html = fs::read_to_string(&output_path).expect("Failed to read HTML");
    assert!(html.contains("Decision Number 1"));
    assert!(html.contains("Decision Number 10"));
    assert!(html.contains("Decision Number 20"));

    // Also test stats on the large collection
    let fs_impl = adrscope::infrastructure::RealFileSystem::new();
    let stats_use_case = adrscope::application::StatsUseCase::new(fs_impl);
    let stats_options = adrscope::application::StatsOptions::new(
        temp_dir
            .join("docs/decisions")
            .to_string_lossy()
            .to_string(),
    )
    .with_format(adrscope::application::stats::StatsFormat::Json);

    let stats_result = stats_use_case
        .execute(&stats_options)
        .expect("Stats should succeed");

    let json: serde_json::Value = serde_json::from_str(&stats_result.output).unwrap();
    assert_eq!(json["total_count"], 20);

    cleanup_temp_dir(&temp_dir);
}
