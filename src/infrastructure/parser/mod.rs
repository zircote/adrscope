//! ADR parsing infrastructure.
//!
//! This module provides parsers for extracting frontmatter and converting
//! markdown to HTML.

mod frontmatter;
mod markdown;

use std::path::Path;

use crate::domain::{Adr, AdrId};
use crate::error::Result;

pub use frontmatter::FrontmatterParser;
pub use markdown::MarkdownRenderer;

/// Trait for parsing ADR files.
pub trait AdrParser: Send + Sync {
    /// Parses an ADR from file contents.
    fn parse(&self, path: &Path, content: &str) -> Result<Adr>;
}

/// Default ADR parser implementation.
#[derive(Debug, Clone, Default)]
pub struct DefaultAdrParser {
    frontmatter_parser: FrontmatterParser,
    markdown_renderer: MarkdownRenderer,
}

impl DefaultAdrParser {
    /// Creates a new default ADR parser.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl AdrParser for DefaultAdrParser {
    fn parse(&self, path: &Path, content: &str) -> Result<Adr> {
        // Extract ID from filename
        let id = AdrId::from_path(path);

        // Extract filename
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown.md")
            .to_string();

        // Parse frontmatter and get body
        let (frontmatter, body_markdown) = self.frontmatter_parser.parse(path, content)?;

        // Render markdown to HTML
        let body_html = self.markdown_renderer.render(body_markdown);

        // Extract plain text for search indexing
        let body_text = self.markdown_renderer.render_plain_text(body_markdown);

        Ok(Adr::new(
            id,
            filename,
            path.to_path_buf(),
            frontmatter,
            body_markdown.to_string(),
            body_html,
            body_text,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Status;
    use std::path::PathBuf;

    #[test]
    fn test_parse_full_adr() {
        let content = r#"---
title: Use PostgreSQL for Primary Storage
description: Decision to adopt PostgreSQL as our primary database
status: accepted
category: architecture
tags:
  - database
  - postgresql
author: Architecture Team
created: "2025-01-15"
---

# Context

We need a reliable primary database for our application.

## Decision

We will use PostgreSQL.

## Consequences

PostgreSQL provides the features we need.
"#;

        let parser = DefaultAdrParser::new();
        let path = PathBuf::from("adr_0001.md");
        let adr = parser.parse(&path, content).expect("should parse");

        assert_eq!(adr.id().as_str(), "adr_0001");
        assert_eq!(adr.title(), "Use PostgreSQL for Primary Storage");
        assert_eq!(adr.status(), Status::Accepted);
        assert_eq!(adr.category(), "architecture");
        assert!(adr.body_html().contains("<h1>"));
        assert!(adr.body_text().contains("Context"));
    }

    #[test]
    fn test_parse_minimal_adr() {
        let content = r"---
title: Minimal ADR
---

Simple content.
";

        let parser = DefaultAdrParser::new();
        let path = PathBuf::from("minimal.md");
        let adr = parser.parse(&path, content).expect("should parse");

        assert_eq!(adr.title(), "Minimal ADR");
        assert_eq!(adr.status(), Status::Proposed); // default
    }
}
