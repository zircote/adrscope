//! Statistics generation use case.
//!
//! Orchestrates ADR discovery, parsing, and statistics computation.

use std::path::Path;

use crate::domain::AdrStatistics;
use crate::error::Result;
use crate::infrastructure::{AdrParser, DefaultAdrParser, FileSystem};

/// Output format for statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatsFormat {
    /// Human-readable text format.
    #[default]
    Text,
    /// JSON format.
    Json,
    /// Markdown format.
    Markdown,
}

impl std::str::FromStr for StatsFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "markdown" | "md" => Ok(Self::Markdown),
            _ => Err(format!("invalid format: {s}")),
        }
    }
}

/// Options for the stats command.
#[derive(Debug, Clone)]
pub struct StatsOptions {
    /// Input directory containing ADR files.
    pub input_dir: String,
    /// Glob pattern for matching ADR files.
    pub pattern: String,
    /// Output format.
    pub format: StatsFormat,
}

impl Default for StatsOptions {
    fn default() -> Self {
        Self {
            input_dir: "docs/decisions".to_string(),
            pattern: "**/*.md".to_string(),
            format: StatsFormat::Text,
        }
    }
}

impl StatsOptions {
    /// Creates new options with the given input directory.
    #[must_use]
    pub fn new(input_dir: impl Into<String>) -> Self {
        Self {
            input_dir: input_dir.into(),
            ..Default::default()
        }
    }

    /// Sets the glob pattern for matching files.
    #[must_use]
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = pattern.into();
        self
    }

    /// Sets the output format.
    #[must_use]
    pub const fn with_format(mut self, format: StatsFormat) -> Self {
        self.format = format;
        self
    }
}

/// Use case for generating ADR statistics.
#[derive(Debug)]
pub struct StatsUseCase<F: FileSystem> {
    fs: F,
    parser: DefaultAdrParser,
}

impl<F: FileSystem> StatsUseCase<F> {
    /// Creates a new stats use case.
    #[must_use]
    pub fn new(fs: F) -> Self {
        Self {
            fs,
            parser: DefaultAdrParser::new(),
        }
    }

    /// Executes the statistics generation use case.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No ADR files are found
    /// - File reading fails
    pub fn execute(&self, options: &StatsOptions) -> Result<StatsResult> {
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
        let mut parse_errors = Vec::new();

        for file_path in &files {
            let content = match self.fs.read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    parse_errors.push((file_path.clone(), e));
                    continue;
                },
            };

            match self.parser.parse(file_path, &content) {
                Ok(adr) => adrs.push(adr),
                Err(e) => parse_errors.push((file_path.clone(), e)),
            }
        }

        // Compute statistics
        let statistics = AdrStatistics::from_adrs(&adrs);

        // Format output
        let output = match options.format {
            StatsFormat::Text => statistics.summary(),
            StatsFormat::Json => {
                serde_json::to_string_pretty(&statistics).unwrap_or_else(|_| "{}".to_string())
            },
            StatsFormat::Markdown => format_markdown(&statistics),
        };

        Ok(StatsResult {
            statistics,
            output,
            parse_errors,
        })
    }
}

/// Result of the statistics use case.
#[derive(Debug)]
pub struct StatsResult {
    /// Computed statistics.
    pub statistics: AdrStatistics,
    /// Formatted output string.
    pub output: String,
    /// Files that failed to parse.
    pub parse_errors: Vec<(std::path::PathBuf, crate::error::Error)>,
}

impl StatsResult {
    /// Returns true if there were any parse errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.parse_errors.is_empty()
    }
}

/// Formats statistics as markdown.
fn format_markdown(stats: &AdrStatistics) -> String {
    use std::fmt::Write;
    let mut output = String::new();

    let _ = writeln!(output, " ADR Statistics\n");
    let _ = writeln!(output, "**Total ADRs:** {}\n", stats.total_count);

    let _ = writeln!(output, "# By Status\n");
    let _ = writeln!(output, "| Status | Count |");
    let _ = writeln!(output, "|--------|-------|");
    for (status, count) in &stats.by_status {
        let _ = writeln!(output, "| {status} | {count} |");
    }

    if !stats.by_category.is_empty() {
        let _ = writeln!(output, "\n## By Category\n");
        let _ = writeln!(output, "| Category | Count |");
        let _ = writeln!(output, "|----------|-------|");
        for (category, count) in &stats.by_category {
            let _ = writeln!(output, "| {category} | {count} |");
        }
    }

    if !stats.by_author.is_empty() {
        let _ = writeln!(output, "\n## By Author\n");
        let _ = writeln!(output, "| Author | Count |");
        let _ = writeln!(output, "|--------|-------|");
        for (author, count) in &stats.by_author {
            let _ = writeln!(output, "| {author} | {count} |");
        }
    }

    if let (Some(earliest), Some(latest)) = (&stats.earliest_date, &stats.latest_date) {
        let _ = writeln!(output, "\n## Date Range\n");
        let _ = writeln!(output, "- **Earliest:** {earliest}");
        let _ = writeln!(output, "- **Latest:** {latest}");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::test_support::InMemoryFileSystem;

    fn sample_adr_content(title: &str, status: &str, category: &str) -> String {
        format!(
            r"---
title: {title}
status: {status}
category: {category}
created: 2025-01-15
description: Test ADR
author: Test Author
---

# {title}

Content here.
"
        )
    }

    #[test]
    fn test_stats_success() {
        let fs = InMemoryFileSystem::new();
        fs.add_file(
            "docs/decisions/adr-0001.md",
            &sample_adr_content("ADR 1", "accepted", "database"),
        );
        fs.add_file(
            "docs/decisions/adr-0002.md",
            &sample_adr_content("ADR 2", "proposed", "api"),
        );
        fs.add_file(
            "docs/decisions/adr-0003.md",
            &sample_adr_content("ADR 3", "accepted", "database"),
        );

        let use_case = StatsUseCase::new(fs);
        let options = StatsOptions::new("docs/decisions");

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.statistics.total_count, 3);
        assert!(!result.has_errors());
    }

    #[test]
    fn test_stats_json_format() {
        let fs = InMemoryFileSystem::new();
        fs.add_file(
            "docs/decisions/adr-0001.md",
            &sample_adr_content("ADR 1", "accepted", "database"),
        );

        let use_case = StatsUseCase::new(fs);
        let options = StatsOptions::new("docs/decisions").with_format(StatsFormat::Json);

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.output.contains("\"total_count\""));
    }

    #[test]
    fn test_stats_markdown_format() {
        let fs = InMemoryFileSystem::new();
        fs.add_file(
            "docs/decisions/adr-0001.md",
            &sample_adr_content("ADR 1", "accepted", "database"),
        );

        let use_case = StatsUseCase::new(fs);
        let options = StatsOptions::new("docs/decisions").with_format(StatsFormat::Markdown);

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.output.contains(" ADR Statistics"));
        assert!(result.output.contains("| Status | Count |"));
    }

    #[test]
    fn test_stats_no_adrs() {
        let fs = InMemoryFileSystem::new();
        let use_case = StatsUseCase::new(fs);
        let options = StatsOptions::new("empty/dir");

        let result = use_case.execute(&options);
        assert!(result.is_err());
    }

    #[test]
    fn test_stats_format_from_str() {
        assert_eq!("text".parse::<StatsFormat>().ok(), Some(StatsFormat::Text));
        assert_eq!("json".parse::<StatsFormat>().ok(), Some(StatsFormat::Json));
        assert_eq!(
            "markdown".parse::<StatsFormat>().ok(),
            Some(StatsFormat::Markdown)
        );
        assert_eq!(
            "md".parse::<StatsFormat>().ok(),
            Some(StatsFormat::Markdown)
        );
        assert!("invalid".parse::<StatsFormat>().is_err());
    }

    #[test]
    fn test_stats_options_builder() {
        let options = StatsOptions::new("input")
            .with_pattern("*.md")
            .with_format(StatsFormat::Json);

        assert_eq!(options.input_dir, "input");
        assert_eq!(options.pattern, "*.md");
        assert_eq!(options.format, StatsFormat::Json);
    }
}
