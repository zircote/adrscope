---
title: Use Askama for Compile-Time HTML Template Generation
description: Decision to use Askama for type-safe, compile-time HTML template generation with zero runtime overhead
type: adr
category: tooling, performance
tags:
  - templates
  - html
  - performance
  - type-safety
  - code-generation
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
  - askama
  - html
audience:
  - developers
  - architects
related: []
---

## Context

HTML viewer generation requires template rendering to produce self-contained HTML output for ADR documentation. The implementation must balance maintainability, performance, and developer experience.

Several approaches were evaluated:

- **Runtime template engines (Tera, Handlebars)**: These engines parse and compile templates at runtime, offering flexibility to modify templates without recompilation. However, they introduce runtime parsing overhead and defer template errors to runtime, potentially causing failures in production.

- **Compile-time template engines (Askama)**: Askama compiles templates directly into Rust code at build time, eliminating runtime parsing entirely. Template errors are caught during compilation, and the generated code benefits from Rust's type system and optimizations.

- **String concatenation**: Building HTML through direct string operations is error-prone, difficult to maintain, and lacks any structural validation. Escaping must be handled manually, creating security risks.

Key requirements for the template system:

- Type-safe variable binding with compile-time validation
- Support for embedded CSS and JavaScript in self-contained output
- Efficient rendering with minimal runtime overhead
- Maintainable template syntax with clear separation of concerns

## Decision

We will use **Askama** for compile-time HTML template generation.

### Implementation Approach

#### Template Structs

Define Rust structs that map directly to template variables:

```rust
use askama::Template;

#[derive(Template)]
#[template(path = "viewer.html")]
pub struct ViewerTemplate<'a> {
    pub title: &'a str,
    pub adrs: &'a [AdrSummary],
    pub generated_at: &'a str,
    pub version: &'a str,
}

#[derive(Template)]
#[template(path = "wiki_page.html")]
pub struct WikiPageTemplate<'a> {
    pub adr: &'a AdrDocument,
    pub navigation: &'a Navigation,
}
```

#### Self-Contained Output

Templates embed CSS and JavaScript directly for standalone HTML files:

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        /* Embedded styles */
        {% include "styles.css" %}
    </style>
</head>
<body>
    {{ content }}
    <script>
        /* Embedded scripts */
        {% include "scripts.js" %}
    </script>
</body>
</html>
```

#### Compile-Time Validation

Askama validates all template references at build time:

```rust
// Compile error if 'title' is missing from struct
#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub title: String,  // Must match {{ title }} in template
    pub content: String,
}
```

### Configuration

Add Askama to `Cargo.toml`:

```toml
[dependencies]
askama = "0.12"

[build-dependencies]
askama = "0.12"
```

Templates are stored in the `templates/` directory with automatic discovery.

## Consequences

### Positive

- **Type-safe templates with compile-time error checking**: Template variable references are validated against struct fields during compilation. Typos, missing variables, and type mismatches are caught before the code can run, eliminating an entire class of runtime errors.

- **Zero runtime template parsing overhead**: Templates are compiled directly into optimized Rust code. The rendering path is a series of string writes with no parsing, interpretation, or dynamic dispatch. This provides maximum performance for HTML generation.

- **IDE support for template variables**: Because template variables correspond to Rust struct fields, IDEs provide autocompletion, go-to-definition, and refactoring support. Renaming a field updates all references across the codebase.

- **Automatic HTML escaping**: Askama escapes output by default, preventing XSS vulnerabilities. Raw output requires explicit opt-in with the `|safe` filter, making security the default behavior.

### Negative

- **Template changes require recompilation**: Unlike runtime engines, modifying a template file necessitates a full rebuild. This slows iteration during template development and prevents template updates in deployed applications.

- **Learning curve for Askama syntax**: Developers must learn Askama's Jinja2-inspired template syntax, including control structures, filters, and inheritance. This adds onboarding time for team members unfamiliar with the system.

- **Limited runtime template customization**: End users cannot provide custom templates without rebuilding the application. Use cases requiring user-defined templates would need a different approach.

### Trade-off

This decision prioritizes **performance and type safety** over **runtime flexibility**. The compile-time validation eliminates template-related runtime errors entirely, and the generated code is as fast as hand-written string concatenation. The cost is that template iteration requires recompilation and end-user template customization is not supported.

For a developer tool that ships pre-built templates, this trade-off is appropriate. The templates are owned by the development team, and the performance and reliability benefits outweigh the loss of runtime flexibility.
