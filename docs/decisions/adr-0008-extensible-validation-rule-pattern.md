---
title: Implement Extensible Validation with Rule Trait Pattern
description: Decision to use a trait-based validation rule pattern for composable, extensible ADR validation with severity levels
type: adr
category: architecture, api
tags:
  - validation
  - traits
  - extensibility
  - composition
  - design-patterns
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
  - traits
audience:
  - developers
  - architects
related:
  - adr-0002-clean-architecture-layers.md
---

## Context

ADR validation is a core feature of ADRScope that needs to support multiple validation concerns:

- **Required field validation**: Blocking errors when mandatory fields like `title` and `status` are missing
- **Recommended field validation**: Advisory warnings when best-practice fields like `description`, `created`, and `category` are absent
- **Future extensibility**: Support for custom validation rules such as naming conventions, status workflow enforcement, relationship consistency, and project-specific requirements

The validation system must distinguish between:

- **Errors**: Blocking issues that should fail CI pipelines and prevent invalid ADRs from being accepted
- **Warnings**: Advisory issues that inform users of best practices without blocking workflows

A monolithic validation function would:

- Become difficult to maintain as rules accumulate
- Make it hard to add project-specific rules without modifying core code
- Prevent users from enabling or disabling individual rules
- Complicate testing of individual validation concerns

## Decision

We will implement a **ValidationRule trait pattern** that enables composable, extensible validation through well-defined abstractions.

### The ValidationRule Trait

```rust
pub trait ValidationRule: Send + Sync {
    /// Returns the unique identifier for this rule
    fn name(&self) -> &str;

    /// Returns a human-readable description of what this rule checks
    fn description(&self) -> &str;

    /// Validates the ADR and returns any issues found
    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue>;
}
```

### Severity Levels

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Blocking issue that must be resolved
    Error,
    /// Advisory issue that should be addressed
    Warning,
}
```

### Validation Issue

```rust
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub field: Option<String>,
}
```

### Validator Struct

The `Validator` holds a collection of rules and executes them against ADRs:

```rust
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn with_rule(mut self, rule: Box<dyn ValidationRule>) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn validate(&self, adr: &Adr) -> ValidationReport {
        let issues: Vec<ValidationIssue> = self
            .rules
            .iter()
            .flat_map(|rule| rule.validate(adr))
            .collect();

        ValidationReport::new(issues)
    }
}
```

### Validation Report

```rust
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.severity == Severity::Warning)
    }
}
```

### Built-in Rules

#### RequiredFieldsRule

Checks for mandatory fields and emits errors when missing:

```rust
pub struct RequiredFieldsRule {
    fields: Vec<String>,
}

impl RequiredFieldsRule {
    pub fn new(fields: Vec<String>) -> Self {
        Self { fields }
    }

    pub fn default_fields() -> Self {
        Self::new(vec!["title".into(), "status".into()])
    }
}

impl ValidationRule for RequiredFieldsRule {
    fn name(&self) -> &str { "required-fields" }

    fn description(&self) -> &str {
        "Checks that required frontmatter fields are present"
    }

    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        self.fields
            .iter()
            .filter(|field| !adr.frontmatter().has_field(field))
            .map(|field| ValidationIssue {
                rule: self.name().into(),
                severity: Severity::Error,
                message: format!("Missing required field: {field}"),
                field: Some(field.clone()),
            })
            .collect()
    }
}
```

#### RecommendedFieldsRule

Checks for best-practice fields and emits warnings when missing:

```rust
pub struct RecommendedFieldsRule {
    fields: Vec<String>,
}

impl RecommendedFieldsRule {
    pub fn default_fields() -> Self {
        Self::new(vec![
            "description".into(),
            "created".into(),
            "category".into(),
        ])
    }
}

impl ValidationRule for RecommendedFieldsRule {
    fn name(&self) -> &str { "recommended-fields" }

    fn description(&self) -> &str {
        "Checks that recommended frontmatter fields are present"
    }

    fn validate(&self, adr: &Adr) -> Vec<ValidationIssue> {
        self.fields
            .iter()
            .filter(|field| !adr.frontmatter().has_field(field))
            .map(|field| ValidationIssue {
                rule: self.name().into(),
                severity: Severity::Warning,
                message: format!("Missing recommended field: {field}"),
                field: Some(field.clone()),
            })
            .collect()
    }
}
```

### Default Validator Configuration

```rust
impl Default for Validator {
    fn default() -> Self {
        Self::new()
            .with_rule(Box::new(RequiredFieldsRule::default_fields()))
            .with_rule(Box::new(RecommendedFieldsRule::default_fields()))
    }
}
```

## Consequences

### Positive

- **Easy extensibility**: Adding new validation rules requires only implementing the `ValidationRule` trait without modifying existing code
- **Testable in isolation**: Each rule can be unit tested independently with focused test cases
- **Composable validation**: Users can select which rules to enable for their project by including or excluding rules from the `Validator`
- **Severity differentiation**: Clear distinction between blocking errors and advisory warnings enables flexible CI behavior (e.g., fail on errors, report warnings)
- **Self-documenting**: Each rule provides its own name and description, enabling automatic documentation generation and helpful error messages
- **Configurable field sets**: Required and recommended fields can be customized per project via configuration

### Negative

- **Dynamic dispatch overhead**: Using `Box<dyn ValidationRule>` incurs vtable lookup cost for each rule invocation
  - *Mitigation*: Validation is not performance-critical; the overhead is negligible compared to I/O operations
- **More types to maintain**: Separate structs for `Validator`, `ValidationRule`, `ValidationIssue`, `ValidationReport`, and `Severity` add cognitive load
  - *Mitigation*: Clear naming and single-responsibility design make each type easy to understand

### Trade-offs

- **Extensibility vs. simplicity**: The trait-based pattern adds more code than a simple validation function, but the investment pays off as validation requirements grow and users request customization
- **Runtime flexibility vs. compile-time guarantees**: Dynamic dispatch enables runtime rule composition but loses some compile-time type checking; this trade-off is appropriate for a plugin-like validation system

### Future Extensions

This pattern naturally supports planned features:

- **Custom user rules**: Load validation rules from configuration or plugins
- **Status workflow validation**: Ensure status transitions follow defined workflow (e.g., proposed -> accepted)
- **Relationship validation**: Check that referenced ADRs exist and relationships are bidirectional
- **Naming convention rules**: Validate ADR filenames match expected patterns
- **Content rules**: Check for minimum description length, required sections, etc.
