//! Faceted filtering data structures.
//!
//! Facets provide aggregated counts for filterable fields in the ADR collection,
//! enabling the UI to show filter options with their counts.

use std::collections::HashMap;

use serde::Serialize;

use super::{Adr, Status};

/// A single facet value with its count.
#[derive(Debug, Clone, Serialize)]
pub struct FacetValue {
    /// The value (e.g., "accepted", "architecture", "database").
    pub value: String,
    /// Number of ADRs with this value.
    pub count: usize,
}

impl FacetValue {
    /// Creates a new facet value.
    #[must_use]
    pub fn new(value: impl Into<String>, count: usize) -> Self {
        Self {
            value: value.into(),
            count,
        }
    }
}

/// A facet is a filterable dimension with all its possible values.
#[derive(Debug, Clone, Serialize)]
pub struct Facet {
    /// Name of the facet (e.g., "status", "category").
    pub name: String,
    /// All possible values for this facet, sorted by count descending.
    pub values: Vec<FacetValue>,
}

impl Facet {
    /// Creates a new facet with the given name and values.
    #[must_use]
    pub fn new(name: impl Into<String>, mut values: Vec<FacetValue>) -> Self {
        // Sort by count descending, then alphabetically
        values.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
        Self {
            name: name.into(),
            values,
        }
    }

    /// Creates a facet from a frequency map.
    #[must_use]
    pub fn from_counts(name: impl Into<String>, counts: HashMap<String, usize>) -> Self {
        let values = counts
            .into_iter()
            .map(|(value, count)| FacetValue::new(value, count))
            .collect();
        Self::new(name, values)
    }
}

/// Collection of all facets computed from ADRs.
#[derive(Debug, Clone, Serialize)]
pub struct Facets {
    /// Status facet with all lifecycle states.
    pub statuses: Vec<FacetValue>,
    /// Category facet.
    pub categories: Vec<FacetValue>,
    /// Tags facet.
    pub tags: Vec<FacetValue>,
    /// Authors facet.
    pub authors: Vec<FacetValue>,
    /// Projects facet.
    pub projects: Vec<FacetValue>,
    /// Technologies facet.
    pub technologies: Vec<FacetValue>,
}

impl Facets {
    /// Computes facets from a collection of ADRs.
    #[must_use]
    pub fn from_adrs(adrs: &[Adr]) -> Self {
        let mut statuses: HashMap<String, usize> = HashMap::new();
        let mut categories: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<String, usize> = HashMap::new();
        let mut authors: HashMap<String, usize> = HashMap::new();
        let mut projects: HashMap<String, usize> = HashMap::new();
        let mut technologies: HashMap<String, usize> = HashMap::new();

        // Initialize all status values with 0
        for status in Status::all() {
            statuses.insert(status.as_str().to_string(), 0);
        }

        for adr in adrs {
            // Count status
            *statuses
                .entry(adr.status().as_str().to_string())
                .or_insert(0) += 1;

            // Count category
            if !adr.category().is_empty() {
                *categories.entry(adr.category().to_string()).or_insert(0) += 1;
            }

            // Count tags
            for tag in adr.tags() {
                *tags.entry(tag.clone()).or_insert(0) += 1;
            }

            // Count author
            if !adr.author().is_empty() {
                *authors.entry(adr.author().to_string()).or_insert(0) += 1;
            }

            // Count project
            if !adr.project().is_empty() {
                *projects.entry(adr.project().to_string()).or_insert(0) += 1;
            }

            // Count technologies
            for tech in adr.technologies() {
                *technologies.entry(tech.clone()).or_insert(0) += 1;
            }
        }

        Self {
            statuses: sorted_facet_values(statuses),
            categories: sorted_facet_values(categories),
            tags: sorted_facet_values(tags),
            authors: sorted_facet_values(authors),
            projects: sorted_facet_values(projects),
            technologies: sorted_facet_values(technologies),
        }
    }
}

/// Converts a count map to sorted facet values.
fn sorted_facet_values(counts: HashMap<String, usize>) -> Vec<FacetValue> {
    let mut values: Vec<_> = counts
        .into_iter()
        .map(|(value, count)| FacetValue::new(value, count))
        .collect();
    values.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facet_value_creation() {
        let fv = FacetValue::new("accepted", 10);
        assert_eq!(fv.value, "accepted");
        assert_eq!(fv.count, 10);
    }

    #[test]
    fn test_facet_sorting() {
        let values = vec![
            FacetValue::new("a", 1),
            FacetValue::new("b", 5),
            FacetValue::new("c", 3),
        ];
        let facet = Facet::new("test", values);

        assert_eq!(facet.values[0].value, "b"); // count 5
        assert_eq!(facet.values[1].value, "c"); // count 3
        assert_eq!(facet.values[2].value, "a"); // count 1
    }

    #[test]
    fn test_facet_from_counts() {
        let mut counts = HashMap::new();
        counts.insert("proposed".to_string(), 5);
        counts.insert("accepted".to_string(), 10);

        let facet = Facet::from_counts("status", counts);

        assert_eq!(facet.name, "status");
        assert_eq!(facet.values[0].value, "accepted");
        assert_eq!(facet.values[0].count, 10);
    }

    #[test]
    fn test_sorted_facet_values_alphabetical_tie() {
        let mut counts = HashMap::new();
        counts.insert("zebra".to_string(), 5);
        counts.insert("apple".to_string(), 5);

        let values = sorted_facet_values(counts);

        // Same count, should be alphabetically sorted
        assert_eq!(values[0].value, "apple");
        assert_eq!(values[1].value, "zebra");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_facets_from_adrs_with_all_fields() {
        use crate::domain::{Adr, AdrId, Frontmatter, Status};
        use std::path::PathBuf;

        // Create ADRs with all facet fields populated
        let frontmatter1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_category("architecture")
            .with_author("Alice")
            .with_project("project-alpha")
            .with_tags(vec!["database".to_string(), "performance".to_string()])
            .with_technologies(vec!["rust".to_string(), "postgres".to_string()]);

        let frontmatter2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_category("api")
            .with_author("Bob")
            .with_project("project-beta")
            .with_tags(vec!["rest".to_string(), "database".to_string()])
            .with_technologies(vec!["rust".to_string(), "redis".to_string()]);

        let adr1 = Adr::new(
            AdrId::new("adr_0001"),
            "adr_0001.md".to_string(),
            PathBuf::from("adr_0001.md"),
            frontmatter1,
            String::new(),
            String::new(),
            String::new(),
        );

        let adr2 = Adr::new(
            AdrId::new("adr_0002"),
            "adr_0002.md".to_string(),
            PathBuf::from("adr_0002.md"),
            frontmatter2,
            String::new(),
            String::new(),
            String::new(),
        );

        let facets = Facets::from_adrs(&[adr1, adr2]);

        // Check statuses
        assert!(
            facets
                .statuses
                .iter()
                .any(|f| f.value == "accepted" && f.count == 1)
        );
        assert!(
            facets
                .statuses
                .iter()
                .any(|f| f.value == "proposed" && f.count == 1)
        );

        // Check categories
        assert!(
            facets
                .categories
                .iter()
                .any(|f| f.value == "architecture" && f.count == 1)
        );
        assert!(
            facets
                .categories
                .iter()
                .any(|f| f.value == "api" && f.count == 1)
        );

        // Check authors
        assert!(
            facets
                .authors
                .iter()
                .any(|f| f.value == "Alice" && f.count == 1)
        );
        assert!(
            facets
                .authors
                .iter()
                .any(|f| f.value == "Bob" && f.count == 1)
        );

        // Check projects
        assert!(
            facets
                .projects
                .iter()
                .any(|f| f.value == "project-alpha" && f.count == 1)
        );
        assert!(
            facets
                .projects
                .iter()
                .any(|f| f.value == "project-beta" && f.count == 1)
        );

        // Check tags (database appears in both ADRs)
        assert!(
            facets
                .tags
                .iter()
                .any(|f| f.value == "database" && f.count == 2)
        );

        // Check technologies (rust appears in both ADRs)
        assert!(
            facets
                .technologies
                .iter()
                .any(|f| f.value == "rust" && f.count == 2)
        );
        assert!(
            facets
                .technologies
                .iter()
                .any(|f| f.value == "postgres" && f.count == 1)
        );
        assert!(
            facets
                .technologies
                .iter()
                .any(|f| f.value == "redis" && f.count == 1)
        );
    }
}
