//! ADR validation system with extensible rules.
//!
//! This module provides a validation framework for checking ADRs against
//! configurable rules, producing detailed reports.

use std::path::PathBuf;

use super::Adr;

/// Severity level for validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Non-blocking advisory.
    Warning,
    /// Blocking error.
    Error,
}

impl Severity {
    /// Returns the severity as a string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A single validation issue found in an ADR.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity of the issue.
    pub severity: Severity,
    /// Path to the ADR file.
    pub path: PathBuf,
    /// Human-readable description of the issue.
    pub message: String,
    /// Optional line number where the issue was found.
    pub line: Option<usize>,
    /// Name of the rule that produced this issue.
    pub rule: String,
}

impl ValidationIssue {
    /// Creates a new validation issue.
    #[must_use]
    pub fn new(
        severity: Severity,
        path: PathBuf,
        message: impl Into<String>,
        rule: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            path,
            message: message.into(),
            line: None,
            rule: rule.into(),
        }
    }

    /// Creates an error issue.
    #[must_use]
    pub fn error(path: PathBuf, message: impl Into<String>, rule: impl Into<String>) -> Self {
        Self::new(Severity::Error, path, message, rule)
    }

    /// Creates a warning issue.
    #[must_use]
    pub fn warning(path: PathBuf, message: impl Into<String>, rule: impl Into<String>) -> Self {
        Self::new(Severity::Warning, path, message, rule)
    }

    /// Sets the line number.
    #[must_use]
    pub const fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = self.line.map_or_else(String::new, |l| format!(":{l}"));
        write!(
            f,
            "{}: {}{}: {} [{}]",
            self.severity,
            self.path.display(),
            location,
            self.message,
            self.rule
        )
    }
}

/// Aggregated result of validating a collection of ADRs.
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    /// All issues found during validation.
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Creates a new empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an issue to the report.
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Adds multiple issues to the report.
    pub fn add_issues(&mut self, issues: impl IntoIterator<Item = ValidationIssue>) {
        self.issues.extend(issues);
    }

    /// Returns all issues.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Returns issues filtered by severity.
    #[must_use]
    pub fn issues_by_severity(&self, severity: Severity) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == severity)
            .collect()
    }

    /// Returns the count of error-level issues.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors().len()
    }

    /// Returns the count of warning-level issues.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.warnings().len()
    }

    /// Returns error-level issues.
    #[must_use]
    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .collect()
    }

    /// Returns warning-level issues.
    #[must_use]
    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .collect()
    }

    /// Returns true if there are any error-level issues.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Returns true if validation passed (no errors).
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Returns true if the report is empty (no issues at all).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns the total number of issues.
    #[must_use]
    pub fn len(&self) -> usize {
        self.issues.len()
    }

    /// Merges another report into this one.
    pub fn merge(&mut self, other: Self) {
        self.issues.extend(other.issues);
    }
}

/// Trait for implementing validation rules.
///
/// Each rule should be focused and check for a specific condition.
pub trait ValidationRule: Send + Sync {
    /// Returns the human-readable name of this rule.
    fn name(&self) -> &str;

    /// Returns a description of what this rule checks.
    fn description(&self) -> &str;

    /// Validates a single ADR, appending any issues to the report.
    fn validate(&self, adr: &Adr, report: &mut ValidationReport);
}

/// A validator that runs multiple rules against ADRs.
#[derive(Default)]
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    /// Creates a new validator with the given rules.
    #[must_use]
    pub fn new(rules: Vec<Box<dyn ValidationRule>>) -> Self {
        Self { rules }
    }

    /// Adds a rule to the validator.
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    /// Validates a single ADR using all configured rules.
    #[must_use]
    pub fn validate(&self, adr: &Adr) -> ValidationReport {
        let mut report = ValidationReport::new();
        for rule in &self.rules {
            rule.validate(adr, &mut report);
        }
        report
    }

    /// Validates a collection of ADRs using all configured rules.
    #[must_use]
    pub fn validate_all(&self, adrs: &[Adr]) -> ValidationReport {
        let mut report = ValidationReport::new();

        for adr in adrs {
            for rule in &self.rules {
                rule.validate(adr, &mut report);
            }
        }

        report
    }

    /// Returns the configured rules.
    #[must_use]
    pub fn rules(&self) -> &[Box<dyn ValidationRule>] {
        &self.rules
    }
}

// ============================================================================
// Built-in validation rules
// ============================================================================

/// Rule that checks for required frontmatter fields.
#[derive(Debug, Clone, Copy, Default)]
pub struct RequiredFieldsRule;

impl RequiredFieldsRule {
    /// Creates a new required fields rule.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ValidationRule for RequiredFieldsRule {
    fn name(&self) -> &str {
        "required-fields"
    }

    fn description(&self) -> &str {
        "Checks that required frontmatter fields are present"
    }

    fn validate(&self, adr: &Adr, report: &mut ValidationReport) {
        if adr.title().is_empty() {
            report.add_issue(ValidationIssue::error(
                adr.source_path().clone(),
                "missing required field 'title'",
                self.name(),
            ));
        }
    }
}

/// Rule that warns about missing optional but recommended fields.
#[derive(Debug, Clone, Copy, Default)]
pub struct RecommendedFieldsRule;

impl RecommendedFieldsRule {
    /// Creates a new recommended fields rule.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ValidationRule for RecommendedFieldsRule {
    fn name(&self) -> &str {
        "recommended-fields"
    }

    fn description(&self) -> &str {
        "Warns about missing recommended fields"
    }

    fn validate(&self, adr: &Adr, report: &mut ValidationReport) {
        if adr.description().is_empty() {
            report.add_issue(ValidationIssue::warning(
                adr.source_path().clone(),
                "missing recommended field 'description'",
                self.name(),
            ));
        }

        if adr.created().is_none() {
            report.add_issue(ValidationIssue::warning(
                adr.source_path().clone(),
                "missing recommended field 'created'",
                self.name(),
            ));
        }

        if adr.category().is_empty() {
            report.add_issue(ValidationIssue::warning(
                adr.source_path().clone(),
                "missing recommended field 'category'",
                self.name(),
            ));
        }
    }
}

/// Returns the default set of validation rules.
#[must_use]
pub fn default_rules() -> Vec<Box<dyn ValidationRule>> {
    vec![
        Box::new(RequiredFieldsRule),
        Box::new(RecommendedFieldsRule),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AdrId, Frontmatter};
    use std::path::PathBuf;

    fn create_test_adr(title: &str) -> Adr {
        let frontmatter = Frontmatter::new(title);
        Adr::new(
            AdrId::new("test"),
            "test.md".to_string(),
            PathBuf::from("test.md"),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        )
    }

    #[test]
    fn test_validation_issue_display() {
        let issue =
            ValidationIssue::error(PathBuf::from("test.md"), "missing title", "required-fields");
        let display = issue.to_string();
        assert!(display.contains("error:"));
        assert!(display.contains("test.md"));
        assert!(display.contains("missing title"));
        assert!(display.contains("[required-fields]"));
    }

    #[test]
    fn test_validation_issue_with_line() {
        let issue = ValidationIssue::warning(
            PathBuf::from("test.md"),
            "missing description",
            "recommended",
        )
        .with_line(5);

        let display = issue.to_string();
        assert!(display.contains(":5:"));
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert!(report.is_empty());
        assert!(report.is_valid());

        report.add_issue(ValidationIssue::warning(
            PathBuf::from("a.md"),
            "warning 1",
            "test",
        ));
        assert!(!report.is_empty());
        assert!(report.is_valid()); // Warnings don't fail validation

        report.add_issue(ValidationIssue::error(
            PathBuf::from("b.md"),
            "error 1",
            "test",
        ));
        assert!(!report.is_valid());
        assert!(report.has_errors());

        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.len(), 2);
    }

    #[test]
    fn test_required_fields_rule() {
        let rule = RequiredFieldsRule;
        let mut report = ValidationReport::new();

        // ADR with title should pass
        let adr = create_test_adr("Test Title");
        rule.validate(&adr, &mut report);
        assert!(report.is_valid());

        // ADR without title should fail
        let mut report = ValidationReport::new();
        let adr = create_test_adr("");
        rule.validate(&adr, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_validator_with_multiple_rules() {
        let validator = Validator::new(default_rules());
        let adr = create_test_adr("Test");
        let report = validator.validate_all(&[adr]);

        // Should have warnings for missing description, created, category
        assert!(report.warning_count() > 0);
    }

    #[test]
    fn test_validation_report_add_issues() {
        let mut report = ValidationReport::new();

        let issues = vec![
            ValidationIssue::error(PathBuf::from("a.md"), "error 1", "test"),
            ValidationIssue::warning(PathBuf::from("b.md"), "warning 1", "test"),
            ValidationIssue::error(PathBuf::from("c.md"), "error 2", "test"),
        ];

        report.add_issues(issues);

        assert_eq!(report.len(), 3);
        assert_eq!(report.error_count(), 2);
        assert_eq!(report.warning_count(), 1);
    }

    #[test]
    fn test_validation_report_issues_accessor() {
        let mut report = ValidationReport::new();

        report.add_issue(ValidationIssue::error(
            PathBuf::from("test.md"),
            "error message",
            "test-rule",
        ));

        let issues = report.issues();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].message, "error message");
        assert_eq!(issues[0].rule, "test-rule");
    }

    #[test]
    fn test_validation_report_issues_by_severity() {
        let mut report = ValidationReport::new();

        report.add_issue(ValidationIssue::error(
            PathBuf::from("a.md"),
            "error 1",
            "test",
        ));
        report.add_issue(ValidationIssue::warning(
            PathBuf::from("b.md"),
            "warning 1",
            "test",
        ));
        report.add_issue(ValidationIssue::error(
            PathBuf::from("c.md"),
            "error 2",
            "test",
        ));

        let errors = report.issues_by_severity(Severity::Error);
        assert_eq!(errors.len(), 2);

        let warnings = report.issues_by_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_required_fields_rule_metadata() {
        let rule = RequiredFieldsRule::new();
        assert_eq!(rule.name(), "required-fields");
        assert!(!rule.description().is_empty());
    }

    #[test]
    fn test_recommended_fields_rule_metadata() {
        let rule = RecommendedFieldsRule::new();
        assert_eq!(rule.name(), "recommended-fields");
        assert!(!rule.description().is_empty());
    }

    #[test]
    fn test_recommended_fields_rule_validation() {
        let rule = RecommendedFieldsRule::new();
        let mut report = ValidationReport::new();

        // ADR with no description, category, or created date
        let frontmatter = Frontmatter::new("Test ADR");
        let adr = Adr::new(
            AdrId::new("test"),
            "test.md".to_string(),
            PathBuf::from("test.md"),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        );

        rule.validate(&adr, &mut report);

        // Should have warnings for description, created, and category
        assert_eq!(report.warning_count(), 3);
    }
}
