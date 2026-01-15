//! Core ADR domain entity.
//!
//! This module defines the `Adr` struct which represents a fully parsed
//! Architecture Decision Record with all its metadata and content.

use std::path::PathBuf;

use serde::Serialize;

use super::{Frontmatter, Status};

/// Unique identifier for an ADR, typically derived from the filename.
///
/// # Examples
///
/// ```
/// use adrscope::domain::AdrId;
///
/// let id = AdrId::new("adr_0001");
/// assert_eq!(id.as_str(), "adr_0001");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AdrId(String);

impl AdrId {
    /// Creates a new ADR identifier.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extracts an ADR ID from a file path.
    ///
    /// The ID is derived from the file stem (filename without extension).
    #[must_use]
    pub fn from_path(path: &std::path::Path) -> Self {
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        Self::new(id)
    }
}

impl std::fmt::Display for AdrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AdrId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A fully parsed Architecture Decision Record.
///
/// Contains the parsed frontmatter metadata, the raw markdown body,
/// and the pre-rendered HTML body for embedding in viewers.
#[derive(Debug, Clone, Serialize)]
pub struct Adr {
    /// Unique identifier derived from filename.
    id: AdrId,

    /// Original filename of the ADR.
    filename: String,

    /// Source file path (relative to ADR directory).
    #[serde(skip)]
    source_path: PathBuf,

    /// Parsed YAML frontmatter.
    frontmatter: Frontmatter,

    /// Raw markdown body (without frontmatter).
    #[serde(skip)]
    body_markdown: String,

    /// Pre-rendered HTML body.
    body_html: String,

    /// Plain text version of body (for search indexing).
    body_text: String,
}

impl Adr {
    /// Creates a new ADR with all components.
    #[must_use]
    pub fn new(
        id: AdrId,
        filename: String,
        source_path: PathBuf,
        frontmatter: Frontmatter,
        body_markdown: String,
        body_html: String,
        body_text: String,
    ) -> Self {
        Self {
            id,
            filename,
            source_path,
            frontmatter,
            body_markdown,
            body_html,
            body_text,
        }
    }

    /// Returns the unique identifier.
    #[must_use]
    pub fn id(&self) -> &AdrId {
        &self.id
    }

    /// Returns the filename.
    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Returns the source file path.
    #[must_use]
    pub fn source_path(&self) -> &PathBuf {
        &self.source_path
    }

    /// Returns the parsed frontmatter.
    #[must_use]
    pub fn frontmatter(&self) -> &Frontmatter {
        &self.frontmatter
    }

    /// Returns the raw markdown body.
    #[must_use]
    pub fn body_markdown(&self) -> &str {
        &self.body_markdown
    }

    /// Returns the pre-rendered HTML body.
    #[must_use]
    pub fn body_html(&self) -> &str {
        &self.body_html
    }

    /// Returns the plain text body for search indexing.
    #[must_use]
    pub fn body_text(&self) -> &str {
        &self.body_text
    }

    // Convenience accessors delegating to frontmatter

    /// Returns the ADR title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Returns the ADR description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.frontmatter.description
    }

    /// Returns the ADR status.
    #[must_use]
    pub fn status(&self) -> Status {
        self.frontmatter.status
    }

    /// Returns the ADR category.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.frontmatter.category
    }

    /// Returns the ADR tags.
    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.frontmatter.tags
    }

    /// Returns the ADR author.
    #[must_use]
    pub fn author(&self) -> &str {
        &self.frontmatter.author
    }

    /// Returns the ADR project.
    #[must_use]
    pub fn project(&self) -> &str {
        &self.frontmatter.project
    }

    /// Returns the technologies affected by this ADR.
    #[must_use]
    pub fn technologies(&self) -> &[String] {
        &self.frontmatter.technologies
    }

    /// Returns the related ADR filenames.
    #[must_use]
    pub fn related(&self) -> &[String] {
        &self.frontmatter.related
    }

    /// Returns the created date if available.
    #[must_use]
    pub fn created(&self) -> Option<time::Date> {
        self.frontmatter.created
    }

    /// Returns the updated date if available.
    #[must_use]
    pub fn updated(&self) -> Option<time::Date> {
        self.frontmatter.updated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adr_id_from_path() {
        let path = PathBuf::from("docs/decisions/adr_0001.md");
        let id = AdrId::from_path(&path);
        assert_eq!(id.as_str(), "adr_0001");
    }

    #[test]
    fn test_adr_id_display() {
        let id = AdrId::new("adr_0001");
        assert_eq!(format!("{id}"), "adr_0001");
    }

    #[test]
    fn test_adr_creation() {
        let frontmatter = Frontmatter::new("Test ADR").with_status(Status::Accepted);

        let adr = Adr::new(
            AdrId::new("adr_0001"),
            "adr_0001.md".to_string(),
            PathBuf::from("docs/decisions/adr_0001.md"),
            frontmatter,
            "# Context\n\nSome context.".to_string(),
            "<h1>Context</h1><p>Some context.</p>".to_string(),
            "Context Some context.".to_string(),
        );

        assert_eq!(adr.id().as_str(), "adr_0001");
        assert_eq!(adr.title(), "Test ADR");
        assert_eq!(adr.status(), Status::Accepted);
        assert!(adr.body_html().contains("<h1>Context</h1>"));
    }

    #[test]
    fn test_adr_serialization() {
        let frontmatter = Frontmatter::new("Test").with_category("architecture");

        let adr = Adr::new(
            AdrId::new("test"),
            "test.md".to_string(),
            PathBuf::from("test.md"),
            frontmatter,
            "body".to_string(),
            "<p>body</p>".to_string(),
            "body".to_string(),
        );

        let json = serde_json::to_string(&adr).expect("should serialize");
        assert!(json.contains("\"id\":\"test\""));
        assert!(json.contains("\"filename\":\"test.md\""));
        // source_path and body_markdown should be skipped
        assert!(!json.contains("source_path"));
        assert!(!json.contains("body_markdown"));
    }

    #[test]
    fn test_adr_all_accessors() {
        use time::macros::date;

        let frontmatter = Frontmatter::new("Complete ADR")
            .with_description("Full description")
            .with_status(Status::Deprecated)
            .with_category("security")
            .with_author("Security Team")
            .with_project("test-project")
            .with_created(date!(2025 - 01 - 10))
            .with_updated(date!(2025 - 01 - 15))
            .with_tags(vec!["security".to_string()])
            .with_technologies(vec!["rust".to_string()])
            .with_related(vec!["adr-001.md".to_string()]);

        let adr = Adr::new(
            AdrId::new("adr_0002"),
            "adr_0002.md".to_string(),
            PathBuf::from("docs/decisions/adr_0002.md"),
            frontmatter,
            "# Body\n\nMarkdown content.".to_string(),
            "<h1>Body</h1><p>Markdown content.</p>".to_string(),
            "Body Markdown content.".to_string(),
        );

        // Test all accessors
        assert_eq!(adr.id().as_str(), "adr_0002");
        assert_eq!(adr.filename(), "adr_0002.md");
        assert_eq!(
            adr.source_path(),
            &PathBuf::from("docs/decisions/adr_0002.md")
        );
        assert_eq!(adr.frontmatter().title, "Complete ADR");
        assert_eq!(adr.body_markdown(), "# Body\n\nMarkdown content.");
        assert_eq!(adr.body_html(), "<h1>Body</h1><p>Markdown content.</p>");
        assert_eq!(adr.body_text(), "Body Markdown content.");
        assert_eq!(adr.title(), "Complete ADR");
        assert_eq!(adr.description(), "Full description");
        assert_eq!(adr.status(), Status::Deprecated);
        assert_eq!(adr.category(), "security");
        assert_eq!(adr.tags(), &["security"]);
        assert_eq!(adr.author(), "Security Team");
        assert_eq!(adr.project(), "test-project");
        assert_eq!(adr.technologies(), &["rust"]);
        assert_eq!(adr.related(), &["adr-001.md"]);
        assert_eq!(adr.created(), Some(date!(2025 - 01 - 10)));
        assert_eq!(adr.updated(), Some(date!(2025 - 01 - 15)));
    }
}
