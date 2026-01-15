//! Generate HTML viewer use case.
//!
//! Orchestrates ADR discovery, parsing, and HTML generation.

use std::path::Path;

use crate::domain::Adr;
use crate::error::Result;
use crate::infrastructure::{
    AdrParser, DefaultAdrParser, FileSystem, HtmlRenderer, RenderConfig, Theme,
};

/// Options for the generate command.
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Input directory containing ADR files.
    pub input_dir: String,
    /// Output file path for the HTML viewer.
    pub output: String,
    /// Page title.
    pub title: String,
    /// Theme preference.
    pub theme: Theme,
    /// Glob pattern for matching ADR files.
    pub pattern: String,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            input_dir: "docs/decisions".to_string(),
            output: "adrs.html".to_string(),
            title: "Architecture Decision Records".to_string(),
            theme: Theme::Auto,
            pattern: "**/*.md".to_string(),
        }
    }
}

impl GenerateOptions {
    /// Creates new options with the given input directory.
    #[must_use]
    pub fn new(input_dir: impl Into<String>) -> Self {
        Self {
            input_dir: input_dir.into(),
            ..Default::default()
        }
    }

    /// Sets the output file path.
    #[must_use]
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.output = output.into();
        self
    }

    /// Sets the page title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the theme preference.
    #[must_use]
    pub const fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the glob pattern for matching files.
    #[must_use]
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = pattern.into();
        self
    }
}

/// Use case for generating HTML viewers.
#[derive(Debug)]
pub struct GenerateUseCase<F: FileSystem> {
    fs: F,
    parser: DefaultAdrParser,
    renderer: HtmlRenderer,
}

impl<F: FileSystem> GenerateUseCase<F> {
    /// Creates a new generate use case.
    #[must_use]
    pub fn new(fs: F) -> Self {
        Self {
            fs,
            parser: DefaultAdrParser::new(),
            renderer: HtmlRenderer::new(),
        }
    }

    /// Executes the generate use case.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No ADR files are found
    /// - File reading fails
    /// - Parsing fails
    /// - HTML generation fails
    /// - File writing fails
    pub fn execute(&self, options: &GenerateOptions) -> Result<GenerateResult> {
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

        // Generate HTML
        let config = RenderConfig::new(&options.title).with_theme(options.theme);
        let html = self
            .renderer
            .render(adrs.clone(), &options.input_dir, &config)?;

        // Write output
        if let Some(parent) = Path::new(&options.output).parent() {
            if !parent.as_os_str().is_empty() {
                self.fs.create_dir_all(parent)?;
            }
        }
        self.fs.write(Path::new(&options.output), &html)?;

        Ok(GenerateResult {
            output_path: options.output.clone(),
            adr_count: adrs.len(),
            parse_errors: errors,
        })
    }

    fn parse_adr(&self, path: &Path) -> Result<Adr> {
        let content = self.fs.read_to_string(path)?;
        self.parser.parse(path, &content)
    }
}

/// Result of the generate use case.
#[derive(Debug)]
pub struct GenerateResult {
    /// Path to the generated HTML file.
    pub output_path: String,
    /// Number of ADRs included.
    pub adr_count: usize,
    /// Files that failed to parse.
    pub parse_errors: Vec<(std::path::PathBuf, crate::error::Error)>,
}

impl GenerateResult {
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
    fn test_generate_success() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", sample_adr_content());

        let use_case = GenerateUseCase::new(fs);
        let options = GenerateOptions::new("docs/decisions").with_output("output.html");

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.adr_count, 1);
        assert_eq!(result.output_path, "output.html");
        assert!(!result.has_errors());
    }

    #[test]
    fn test_generate_no_adrs() {
        let fs = InMemoryFileSystem::new();
        let use_case = GenerateUseCase::new(fs);
        let options = GenerateOptions::new("empty/dir");

        let result = use_case.execute(&options);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_options_builder() {
        let options = GenerateOptions::new("input")
            .with_output("out.html")
            .with_title("My ADRs")
            .with_theme(Theme::Dark)
            .with_pattern("*.md");

        assert_eq!(options.input_dir, "input");
        assert_eq!(options.output, "out.html");
        assert_eq!(options.title, "My ADRs");
        assert_eq!(options.theme, Theme::Dark);
        assert_eq!(options.pattern, "*.md");
    }
}
