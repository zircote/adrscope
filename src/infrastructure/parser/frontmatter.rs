//! YAML frontmatter parsing.
//!
//! Extracts and parses the YAML frontmatter block from ADR files.

use std::path::Path;

use crate::domain::Frontmatter;
use crate::error::{Error, Result};

/// Parser for YAML frontmatter in ADR files.
#[derive(Debug, Clone, Default)]
pub struct FrontmatterParser;

impl FrontmatterParser {
    /// Creates a new frontmatter parser.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Parses frontmatter from file content, returning the frontmatter and remaining body.
    pub fn parse<'a>(&self, path: &Path, content: &'a str) -> Result<(Frontmatter, &'a str)> {
        let (yaml, body) =
            extract_frontmatter(content).ok_or_else(|| Error::InvalidFrontmatter {
                path: path.to_path_buf(),
                message: "missing or invalid frontmatter delimiters (---)".to_string(),
            })?;

        let frontmatter: Frontmatter =
            serde_yaml::from_str(yaml).map_err(|source| Error::YamlParse {
                path: path.to_path_buf(),
                source,
            })?;

        // Validate required fields
        if frontmatter.title.is_empty() {
            return Err(Error::MissingField {
                path: path.to_path_buf(),
                field: "title",
            });
        }

        Ok((frontmatter, body))
    }
}

/// Extracts the YAML frontmatter block and remaining body from content.
///
/// Returns `None` if the content doesn't start with `---` or doesn't have
/// a closing `---` delimiter.
fn extract_frontmatter(content: &str) -> Option<(&str, &str)> {
    // Content must start with "---"
    let content = content.strip_prefix("---")?;

    // Find the closing "---"
    // We need to handle both "---\n" and just "---" at end
    let closing_pos = find_closing_delimiter(content)?;

    let yaml = content[..closing_pos].trim();
    let body = content[closing_pos + 3..].trim_start_matches(['\n', '\r']);

    Some((yaml, body))
}

/// Finds the position of the closing `---` delimiter.
///
/// The closing delimiter must be at the start of a line (after a newline).
fn find_closing_delimiter(content: &str) -> Option<usize> {
    // Look for "\n---" to find a delimiter at the start of a line
    content.find("\n---").map(|pos| pos + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_frontmatter_basic() {
        let content = r"---
title: Test
status: accepted
---
Body content here.
";

        let (yaml, body) = extract_frontmatter(content).expect("should extract");

        assert!(yaml.contains("title: Test"));
        assert!(yaml.contains("status: accepted"));
        assert_eq!(body.trim(), "Body content here.");
    }

    #[test]
    fn test_extract_frontmatter_multiline_body() {
        let content = r"---
title: Test
---
# Heading

Paragraph 1.

Paragraph 2.
";

        let (yaml, body) = extract_frontmatter(content).expect("should extract");

        assert!(yaml.contains("title: Test"));
        assert!(body.contains(" Heading"));
        assert!(body.contains("Paragraph 1."));
    }

    #[test]
    fn test_extract_frontmatter_no_delimiter() {
        let content = "No frontmatter here.";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn test_extract_frontmatter_missing_closing() {
        let content = r"---
title: Test
No closing delimiter
";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn test_parse_frontmatter_success() {
        let content = r"---
title: Use Rust
description: Decision to use Rust for CLI
status: accepted
category: technology
tags:
  - rust
  - cli
author: Team Lead
---
Body here.
";

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let (frontmatter, body) = parser.parse(&path, content).expect("should parse");

        assert_eq!(frontmatter.title, "Use Rust");
        assert_eq!(frontmatter.description, "Decision to use Rust for CLI");
        assert_eq!(frontmatter.category, "technology");
        assert_eq!(frontmatter.tags, vec!["rust", "cli"]);
        assert_eq!(frontmatter.author, "Team Lead");
        assert_eq!(body.trim(), "Body here.");
    }

    #[test]
    fn test_parse_frontmatter_missing_title() {
        let content = r"---
description: Missing title
---
Body
";

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let result = parser.parse(&path, content);

        // serde_yaml returns YamlParse error for missing required fields
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::YamlParse { .. })));
    }

    #[test]
    fn test_parse_frontmatter_empty_title() {
        let content = r#"---
title: ""
---
Body
"#;

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let result = parser.parse(&path, content);

        // Empty title is caught by our validation check
        assert!(result.is_err());
        if let Err(Error::MissingField { field, .. }) = result {
            assert_eq!(field, "title");
        } else {
            panic!("Expected MissingField error, got {:?}", result);
        }
    }

    #[test]
    fn test_parse_frontmatter_invalid_yaml() {
        let content = r"---
title: Test
invalid: [unclosed bracket
---
Body
";

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let result = parser.parse(&path, content);

        assert!(matches!(result, Err(Error::YamlParse { .. })));
    }

    #[test]
    fn test_parse_frontmatter_with_dates() {
        let content = r#"---
title: Test with Dates
created: "2025-01-15"
updated: "2025-01-20"
---
Body
"#;

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let (frontmatter, _) = parser.parse(&path, content).expect("should parse");

        assert!(frontmatter.created.is_some());
        assert!(frontmatter.updated.is_some());
    }

    #[test]
    fn test_parse_frontmatter_with_related() {
        let content = r"---
title: Related ADRs
related:
  - adr_0001.md
  - adr_0005.md
---
Body
";

        let parser = FrontmatterParser::new();
        let path = PathBuf::from("test.md");
        let (frontmatter, _) = parser.parse(&path, content).expect("should parse");

        assert_eq!(frontmatter.related, vec!["adr_0001.md", "adr_0005.md"]);
    }
}
