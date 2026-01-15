//! YAML frontmatter data structure.
//!
//! This module defines the structured-madr frontmatter schema that ADRScope
//! expects in ADR files.

use serde::{Deserialize, Serialize};
use time::Date;

use super::Status;

/// Parsed YAML frontmatter from an ADR file following the structured-madr schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    /// Short descriptive title (1-100 chars).
    pub title: String,

    /// One-sentence summary (1-300 chars).
    #[serde(default)]
    pub description: String,

    /// Document type identifier (const: "adr").
    #[serde(rename = "type", default = "default_type")]
    pub doc_type: String,

    /// Decision category (e.g., architecture, api, security).
    #[serde(default)]
    pub category: String,

    /// Keywords for categorization (kebab-case).
    #[serde(default)]
    pub tags: Vec<String>,

    /// Current status in the lifecycle.
    #[serde(default, deserialize_with = "lenient_status::deserialize")]
    pub status: Status,

    /// ISO 8601 date created.
    #[serde(default, with = "optional_date")]
    pub created: Option<Date>,

    /// ISO 8601 date last modified.
    #[serde(default, with = "optional_date")]
    pub updated: Option<Date>,

    /// Author or team responsible.
    #[serde(default)]
    pub author: String,

    /// Project this decision applies to.
    #[serde(default)]
    pub project: String,

    /// Technologies affected by decision.
    #[serde(default)]
    pub technologies: Vec<String>,

    /// Intended readers.
    #[serde(default)]
    pub audience: Vec<String>,

    /// Filenames of related ADRs.
    #[serde(default)]
    pub related: Vec<String>,
}

fn default_type() -> String {
    "adr".to_string()
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            doc_type: default_type(),
            category: String::new(),
            tags: Vec::new(),
            status: Status::default(),
            created: None,
            updated: None,
            author: String::new(),
            project: String::new(),
            technologies: Vec::new(),
            audience: Vec::new(),
            related: Vec::new(),
        }
    }
}

impl Frontmatter {
    /// Creates a new frontmatter with the given title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the status.
    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Sets the category.
    #[must_use]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Sets the author.
    #[must_use]
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Sets the project.
    #[must_use]
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = project.into();
        self
    }

    /// Sets the created date.
    #[must_use]
    pub const fn with_created(mut self, date: Date) -> Self {
        self.created = Some(date);
        self
    }

    /// Sets the updated date.
    #[must_use]
    pub const fn with_updated(mut self, date: Date) -> Self {
        self.updated = Some(date);
        self
    }

    /// Adds tags.
    #[must_use]
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Adds technologies.
    #[must_use]
    pub fn with_technologies(mut self, technologies: Vec<String>) -> Self {
        self.technologies = technologies;
        self
    }

    /// Adds related ADRs.
    #[must_use]
    pub fn with_related(mut self, related: Vec<String>) -> Self {
        self.related = related;
        self
    }
}

/// Lenient deserialization for Status that warns once per unknown value.
mod lenient_status {
    use std::cell::RefCell;
    use std::collections::HashSet;

    use serde::{Deserialize, Deserializer};

    use super::Status;

    thread_local! {
        /// Track unknown statuses we've already warned about (warn once per unique value per thread).
        static WARNED_STATUSES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) if !s.is_empty() => match s.to_lowercase().as_str() {
                "proposed" => Ok(Status::Proposed),
                "accepted" => Ok(Status::Accepted),
                "deprecated" => Ok(Status::Deprecated),
                "superseded" => Ok(Status::Superseded),
                unknown => {
                    // Only warn once per unique unknown status value per thread
                    WARNED_STATUSES.with(|set| {
                        if set.borrow_mut().insert(unknown.to_string()) {
                            eprintln!(
                                "Warning: Unknown ADR status '{unknown}', defaulting to 'proposed'"
                            );
                        }
                    });
                    Ok(Status::Proposed)
                },
            },
            _ => Ok(Status::default()),
        }
    }
}

/// Custom serialization for optional dates in ISO 8601 format.
mod optional_date {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::{Date, format_description::well_known::Iso8601};

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let s = d
                    .format(&Iso8601::DATE)
                    .map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            },
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) if !s.is_empty() => Date::parse(&s, &Iso8601::DATE)
                .map(Some)
                .map_err(|e| serde::de::Error::custom(format!("invalid date format '{s}': {e}"))),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_default() {
        let fm = Frontmatter::default();
        assert!(fm.title.is_empty());
        assert_eq!(fm.doc_type, "adr");
        assert_eq!(fm.status, Status::Proposed);
    }

    #[test]
    fn test_frontmatter_builder() {
        let fm = Frontmatter::new("Test ADR")
            .with_description("A test decision")
            .with_status(Status::Accepted)
            .with_category("architecture")
            .with_author("Test Team");

        assert_eq!(fm.title, "Test ADR");
        assert_eq!(fm.description, "A test decision");
        assert_eq!(fm.status, Status::Accepted);
        assert_eq!(fm.category, "architecture");
        assert_eq!(fm.author, "Test Team");
    }

    #[test]
    fn test_frontmatter_deserialization() {
        let yaml = r#"
title: Use PostgreSQL
description: Decision to use PostgreSQL for storage
status: accepted
category: architecture
tags:
  - database
  - postgresql
author: Architecture Team
created: "2025-01-15"
"#;
        let fm: Frontmatter = serde_yaml::from_str(yaml).expect("should parse");
        assert_eq!(fm.title, "Use PostgreSQL");
        assert_eq!(fm.status, Status::Accepted);
        assert_eq!(fm.tags, vec!["database", "postgresql"]);
        assert!(fm.created.is_some());
    }

    #[test]
    fn test_frontmatter_serialization() {
        let fm = Frontmatter::new("Test").with_status(Status::Accepted);

        let json = serde_json::to_string(&fm).expect("should serialize");
        assert!(json.contains("\"title\":\"Test\""));
        assert!(json.contains("\"status\":\"accepted\""));
    }

    #[test]
    fn test_frontmatter_builder_all_fields() {
        use time::macros::date;

        let fm = Frontmatter::new("Complete ADR")
            .with_description("Full description")
            .with_status(Status::Deprecated)
            .with_category("security")
            .with_author("Security Team")
            .with_project("my-project")
            .with_created(date!(2025 - 01 - 10))
            .with_updated(date!(2025 - 01 - 15))
            .with_tags(vec!["security".to_string(), "auth".to_string()])
            .with_technologies(vec!["rust".to_string(), "wasm".to_string()])
            .with_related(vec!["adr-001.md".to_string(), "adr-002.md".to_string()]);

        assert_eq!(fm.title, "Complete ADR");
        assert_eq!(fm.description, "Full description");
        assert_eq!(fm.status, Status::Deprecated);
        assert_eq!(fm.category, "security");
        assert_eq!(fm.author, "Security Team");
        assert_eq!(fm.project, "my-project");
        assert_eq!(fm.created, Some(date!(2025 - 01 - 10)));
        assert_eq!(fm.updated, Some(date!(2025 - 01 - 15)));
        assert_eq!(fm.tags, vec!["security", "auth"]);
        assert_eq!(fm.technologies, vec!["rust", "wasm"]);
        assert_eq!(fm.related, vec!["adr-001.md", "adr-002.md"]);
    }

    #[test]
    fn test_frontmatter_date_serialization_roundtrip() {
        use time::macros::date;

        let fm = Frontmatter::new("Date Test")
            .with_created(date!(2025 - 06 - 15))
            .with_updated(date!(2025 - 12 - 25));

        let json = serde_json::to_string(&fm).expect("should serialize");
        assert!(json.contains("2025-06-15"));
        assert!(json.contains("2025-12-25"));

        let roundtrip: Frontmatter = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(roundtrip.created, fm.created);
        assert_eq!(roundtrip.updated, fm.updated);
    }

    #[test]
    fn test_frontmatter_unknown_status_defaults_to_proposed() {
        // Unknown status values should parse successfully with default status
        let yaml = r#"
title: ADR with unknown status
description: This ADR has a non-standard status
status: published
category: architecture
"#;
        let fm: Frontmatter =
            serde_yaml::from_str(yaml).expect("should parse even with unknown status");
        assert_eq!(fm.title, "ADR with unknown status");
        // Unknown status "published" should default to Proposed
        assert_eq!(fm.status, Status::Proposed);
    }

    #[test]
    fn test_frontmatter_missing_status_defaults_to_proposed() {
        let yaml = r#"
title: ADR without status
description: This ADR has no status field
"#;
        let fm: Frontmatter = serde_yaml::from_str(yaml).expect("should parse");
        assert_eq!(fm.status, Status::Proposed);
    }
}
