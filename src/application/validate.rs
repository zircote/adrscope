//! Validate ADRs use case.
//!
//! Orchestrates ADR discovery, parsing, and validation.

use std::path::Path;

use crate::domain::{Severity, ValidationReport, Validator, default_rules};
use crate::error::Result;
use crate::infrastructure::{AdrParser, DefaultAdrParser, FileSystem};

/// Options for the validate command.
#[derive(Debug, Clone)]
pub struct ValidateOptions {
    /// Input directory containing ADR files.
    pub input_dir: String,
    /// Glob pattern for matching ADR files.
    pub pattern: String,
    /// Whether to fail on warnings.
    pub strict: bool,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            input_dir: "docs/decisions".to_string(),
            pattern: "**/*.md".to_string(),
            strict: false,
        }
    }
}

impl ValidateOptions {
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

    /// Sets strict mode (fail on warnings).
    #[must_use]
    pub const fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

/// Use case for validating ADRs.
#[derive(Debug)]
pub struct ValidateUseCase<F: FileSystem> {
    fs: F,
    parser: DefaultAdrParser,
}

impl<F: FileSystem> ValidateUseCase<F> {
    /// Creates a new validate use case.
    #[must_use]
    pub fn new(fs: F) -> Self {
        Self {
            fs,
            parser: DefaultAdrParser::new(),
        }
    }

    /// Executes the validation use case.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No ADR files are found
    /// - File reading fails
    pub fn execute(&self, options: &ValidateOptions) -> Result<ValidateResult> {
        // Discover ADR files
        let base = Path::new(&options.input_dir);
        let files = self.fs.glob(base, &options.pattern)?;

        if files.is_empty() {
            return Err(crate::error::Error::NoAdrsFound {
                path: base.to_path_buf(),
            });
        }

        // Build validator with default rules
        let validator = Validator::new(default_rules());

        // Validate each file
        let mut reports = Vec::with_capacity(files.len());
        let mut parse_errors = Vec::new();

        for file_path in &files {
            match self.validate_file(file_path, &validator) {
                Ok(report) => reports.push((file_path.clone(), report)),
                Err(e) => parse_errors.push((file_path.clone(), e)),
            }
        }

        // Aggregate results
        let mut total_errors = 0;
        let mut total_warnings = 0;

        for (_, report) in &reports {
            total_errors += report.errors().len();
            total_warnings += report.warnings().len();
        }

        // Determine if validation passed
        let passed = if options.strict {
            total_errors == 0 && total_warnings == 0 && parse_errors.is_empty()
        } else {
            total_errors == 0 && parse_errors.is_empty()
        };

        Ok(ValidateResult {
            reports,
            parse_errors,
            total_errors,
            total_warnings,
            passed,
        })
    }

    fn validate_file(&self, path: &Path, validator: &Validator) -> Result<ValidationReport> {
        let content = self.fs.read_to_string(path)?;
        let adr = self.parser.parse(path, &content)?;
        Ok(validator.validate(&adr))
    }
}

/// Result of the validation use case.
#[derive(Debug)]
pub struct ValidateResult {
    /// Validation reports for each successfully parsed file.
    pub reports: Vec<(std::path::PathBuf, ValidationReport)>,
    /// Files that failed to parse.
    pub parse_errors: Vec<(std::path::PathBuf, crate::error::Error)>,
    /// Total number of validation errors.
    pub total_errors: usize,
    /// Total number of validation warnings.
    pub total_warnings: usize,
    /// Whether validation passed.
    pub passed: bool,
}

impl ValidateResult {
    /// Returns all issues (both errors and warnings).
    #[must_use]
    pub fn all_issues(
        &self,
    ) -> impl Iterator<Item = (&std::path::PathBuf, &crate::domain::ValidationIssue)> {
        self.reports
            .iter()
            .flat_map(|(path, report)| report.issues().iter().map(move |issue| (path, issue)))
    }

    /// Returns only error-level issues.
    #[must_use]
    pub fn error_issues(
        &self,
    ) -> impl Iterator<Item = (&std::path::PathBuf, &crate::domain::ValidationIssue)> {
        self.all_issues()
            .filter(|(_, issue)| issue.severity == Severity::Error)
    }

    /// Returns only warning-level issues.
    #[must_use]
    pub fn warning_issues(
        &self,
    ) -> impl Iterator<Item = (&std::path::PathBuf, &crate::domain::ValidationIssue)> {
        self.all_issues()
            .filter(|(_, issue)| issue.severity == Severity::Warning)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::test_support::InMemoryFileSystem;

    fn valid_adr_content() -> &'static str {
        r"---
title: Use PostgreSQL for persistence
status: accepted
category: database
created: 2025-01-15
description: We decided to use PostgreSQL as our primary database.
author: Jane Doe
---

# Use PostgreSQL for persistence

## Context

We need a database.
"
    }

    fn minimal_adr_content() -> &'static str {
        r"---
title: Minimal ADR
status: proposed
---

# Minimal ADR

Some content.
"
    }

    fn invalid_adr_content() -> &'static str {
        r"---
description: Missing title
---

# No Title

Some content.
"
    }

    #[test]
    fn test_validate_valid_adr() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", valid_adr_content());

        let use_case = ValidateUseCase::new(fs);
        let options = ValidateOptions::new("docs/decisions");

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.passed);
        assert_eq!(result.total_errors, 0);
    }

    #[test]
    fn test_validate_minimal_adr_has_warnings() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", minimal_adr_content());

        let use_case = ValidateUseCase::new(fs);
        let options = ValidateOptions::new("docs/decisions");

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        // Passes without strict mode
        assert!(result.passed);
        assert_eq!(result.total_errors, 0);
        // Should have warnings for missing recommended fields
        assert!(result.total_warnings > 0);
    }

    #[test]
    fn test_validate_strict_mode() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", minimal_adr_content());

        let use_case = ValidateUseCase::new(fs);
        let options = ValidateOptions::new("docs/decisions").with_strict(true);

        let result = use_case.execute(&options);
        assert!(result.is_ok());

        let result = result.unwrap();
        // Fails in strict mode due to warnings
        assert!(!result.passed);
    }

    #[test]
    fn test_validate_invalid_adr() {
        let fs = InMemoryFileSystem::new();
        fs.add_file("docs/decisions/adr-0001.md", invalid_adr_content());

        let use_case = ValidateUseCase::new(fs);
        let options = ValidateOptions::new("docs/decisions");

        let result = use_case.execute(&options);
        // Should fail to parse due to missing title
        assert!(
            result.is_err()
                || result
                    .as_ref()
                    .map(|r| !r.parse_errors.is_empty())
                    .unwrap_or(false)
        );
    }

    #[test]
    fn test_validate_no_adrs() {
        let fs = InMemoryFileSystem::new();
        let use_case = ValidateUseCase::new(fs);
        let options = ValidateOptions::new("empty/dir");

        let result = use_case.execute(&options);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_options_builder() {
        let options = ValidateOptions::new("input")
            .with_pattern("*.md")
            .with_strict(true);

        assert_eq!(options.input_dir, "input");
        assert_eq!(options.pattern, "*.md");
        assert!(options.strict);
    }
}
