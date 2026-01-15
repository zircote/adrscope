//! Generate GitHub Wiki pages use case.
//!
//! Orchestrates ADR discovery, parsing, and Wiki markdown generation.

use std::path::Path;

use crate::domain::Adr;
use crate::error::Result;
use crate::infrastructure::renderer::WikiRenderer;
use crate::infrastructure::{AdrParser, DefaultAdrParser, FileSystem};

/// Options for the wiki command.
#[derive(Debug, Clone)]
pub struct WikiOptions {
    /// Input directory containing ADR files.
    pub input_dir: String,
    /// Output directory for wiki files.
    pub output_dir: String,
    /// Optional URL to the GitHub Pages viewer.
    pub pages_url: Option<String>,
    /// Glob pattern for matching ADR files.
    pub pattern: String,
}

impl Default for WikiOptions {
    fn default() -> Self {
        Self {
            input_dir: "docs/decisions".to_string(),
            output_dir: "wiki".to_string(),
            pages_url: None,
            pattern: "**/*.md".to_string(),
        }
    }
}

impl WikiOptions {
    /// Creates new options with the given input directory.
    #[must_use]
    pub fn new(input_dir: impl Into<String>) -> Self {
        Self {
            input_dir: input_dir.into(),
            ..Default::default()
        }
    }

    /// Sets the output directory.
    #[must_use]
    pub fn with_output_dir(mut self, output_dir: impl Into<String>) -> Self {
        self.output_dir = output_dir.into();
        self
    }

    /// Sets the GitHub Pages URL.
    #[must_use]
    pub fn with_pages_url(mut self, url: impl Into<String>) -> Self {
        self.pages_url = Some(url.into());
        self
    }

    /// Sets the glob pattern for matching files.
    #[must_use]
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = pattern.into();
        self
    }
}

/// Use case for generating GitHub Wiki pages.
#[derive(Debug)]
pub struct WikiUseCase<F: FileSystem> {
    fs: F,
    parser: DefaultAdrParser,
    renderer: WikiRenderer,
}

impl<F: FileSystem> WikiUseCase<F> {
    /// Creates a new wiki use case.
    #[must_use]
    pub fn new(fs: F) -> Self {
        Self {
            fs,
            parser: DefaultAdrParser::new(),
            renderer: WikiRenderer::new(),
        }
    }

    /// Executes the wiki generation use case.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No ADR files are found
    /// - File reading fails
    /// - Parsing fails
    /// - File writing fails
    pub fn execute(&self, options: &WikiOptions) -> Result<WikiResult> {
        // Discover ADR files
        let base = Path::new(&options.input_dir);
        let files = self.fs.glob(base, &options.pattern)?;

        if files.is_empty() {
            return Err(crate::error::Error::NoAdrsFound {
                path: base.to_path_buf(),
            });
        }

        // Parse all ADRs
        let mut adrs = Vec::with_capacity(files.len());
        let mut errors = Vec::new();

        for file_path in &files {
            match self.parse_adr(file_path) {
                Ok(adr) => adrs.push(adr),
                Err(e) => errors.push((file_path.clone(), e)),
            }
        }

        // Sort by ID for consistent ordering
        adrs.sort_by(|a, b| a.id().cmp(b.id()));

        // Generate wiki pages
        let pages = self
            .renderer
            .render_all(&adrs, options.pages_url.as_deref())?;

        // Create output directory
        self.fs.create_dir_all(Path::new(&options.output_dir))?;

        // Write wiki pages
        let mut generated_files = Vec::with_capacity(pages.len());
        for (filename, content) in pages {
            let output_path = format!("{}/{}", options.output_dir, filename);
            self.fs.write(Path::new(&output_path), &content)?;
            generated_files.push(output_path);
        }

        // Copy original ADR files to wiki directory
        for adr in &adrs {
            let dest_path = format!("{}/{}", options.output_dir, adr.filename());
            let content = self.fs.read_to_string(adr.source_path())?;
            self.fs.write(Path::new(&dest_path), &content)?;
            generated_files.push(dest_path);
        }

        Ok(WikiResult {
            output_dir: options.output_dir.clone(),
            generated_files,
            adr_count: adrs.len(),
            parse_errors: errors,
        })
    }

    fn parse_adr(&self, path: &Path) -> Result<Adr> {
        let content = self.fs.read_to_string(path)?;
        self.parser.parse(path, &content)
    }
}

/// Result of the wiki generation use case.
#[derive(Debug)]
pub struct WikiResult {
    /// Output directory path.
    pub output_dir: String,
    /// List of generated file paths.
    pub generated_files: Vec<String>,
    /// Number of ADRs processed.
    pub adr_count: usize,
    /// Files that failed to parse.
    pub parse_errors: Vec<(std::path::PathBuf, crate::error::Error)>,
}

impl WikiResult {
    /// Returns true if there were any parse errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.parse_errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::test_support::InMemoryFileSystem;

    fn sample_adr_content() -> &'static str {
        r"---
title: Use PostgreSQL for persistence
status: accepted
category: database
created: 2025-01-15
description: We decided to use PostgreSQL as our primary database.
---

# Use PostgreSQL for persistence

## Context

We need a database for our application.

## Decision

We will use PostgreSQL.

## Consequences

- We get ACID compliance
- We need to manage database migrations
"
    }

    #[test]
    fn test_wiki_success() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", sample_adr_content());

        let use_case = WikiUseCase::new(fs);
        let options = WikiOptions::new("docs/decisions")
            .with_output_dir("wiki")
            .with_pages_url("https://example.com/adrs");

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.adr_count, 1);
        assert!(!result.has_errors());
        // Should have index + by-status + by-category + timeline + statistics + original ADR
        assert!(result.generated_files.len() >= 5);
    }

    #[test]
    fn test_wiki_no_adrs() {
        let fs = InMemoryFileSystem::new();
        let use_case = WikiUseCase::new(fs);
        let options = WikiOptions::new("empty/dir");

        let result = use_case.execute(&options);
        assert!(result.is_err());
    }

    #[test]
    fn test_wiki_options_builder() {
        let options = WikiOptions::new("input")
            .with_output_dir("wiki")
            .with_pages_url("https://example.com")
            .with_pattern("*.md");

        assert_eq!(options.input_dir, "input");
        assert_eq!(options.output_dir, "wiki");
        assert_eq!(options.pages_url, Some("https://example.com".to_string()));
        assert_eq!(options.pattern, "*.md");
    }
}
