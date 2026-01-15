//! Statistics aggregation for ADR collections.
//!
//! This module provides types for computing and representing summary
//! statistics about an ADR collection.

use std::collections::HashMap;

use serde::Serialize;
use time::Date;

use super::{Adr, Status};

/// Aggregated statistics for an ADR collection.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AdrStatistics {
    /// Total number of ADRs.
    pub total_count: usize,
    /// Counts by status.
    pub by_status: HashMap<String, usize>,
    /// Counts by category.
    pub by_category: HashMap<String, usize>,
    /// Counts by author.
    pub by_author: HashMap<String, usize>,
    /// Counts by tag.
    pub by_tag: HashMap<String, usize>,
    /// Counts by technology.
    pub by_technology: HashMap<String, usize>,
    /// Counts by project.
    pub by_project: HashMap<String, usize>,
    /// Counts by year.
    pub by_year: HashMap<i32, usize>,
    /// Earliest created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_date: Option<Date>,
    /// Latest created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_date: Option<Date>,
}

impl AdrStatistics {
    /// Computes statistics from a collection of ADRs.
    #[must_use]
    pub fn from_adrs(adrs: &[Adr]) -> Self {
        let mut stats = Self {
            total_count: adrs.len(),
            ..Self::default()
        };

        // Initialize all status values with 0
        for status in Status::all() {
            stats.by_status.insert(status.as_str().to_string(), 0);
        }

        let mut earliest: Option<Date> = None;
        let mut latest: Option<Date> = None;

        for adr in adrs {
            // Count by status
            *stats
                .by_status
                .entry(adr.status().as_str().to_string())
                .or_insert(0) += 1;

            // Count by category
            if !adr.category().is_empty() {
                *stats
                    .by_category
                    .entry(adr.category().to_string())
                    .or_insert(0) += 1;
            }

            // Count by author
            if !adr.author().is_empty() {
                *stats.by_author.entry(adr.author().to_string()).or_insert(0) += 1;
            }

            // Count by tags
            for tag in adr.tags() {
                *stats.by_tag.entry(tag.clone()).or_insert(0) += 1;
            }

            // Count by technology
            for tech in adr.technologies() {
                *stats.by_technology.entry(tech.clone()).or_insert(0) += 1;
            }

            // Count by project
            if !adr.project().is_empty() {
                *stats
                    .by_project
                    .entry(adr.project().to_string())
                    .or_insert(0) += 1;
            }

            // Track date ranges
            if let Some(created) = adr.created() {
                // Count by year
                *stats.by_year.entry(created.year()).or_insert(0) += 1;

                // Track earliest/latest
                if earliest.is_none_or(|e| created < e) {
                    earliest = Some(created);
                }
                if latest.is_none_or(|l| created > l) {
                    latest = Some(created);
                }
            }
        }

        stats.earliest_date = earliest;
        stats.latest_date = latest;

        stats
    }

    /// Returns the top N items from a count map, sorted by count descending.
    pub fn top_n<S: AsRef<str>>(counts: &HashMap<S, usize>, n: usize) -> Vec<(&str, usize)> {
        let mut items: Vec<_> = counts.iter().map(|(k, &v)| (k.as_ref(), v)).collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.truncate(n);
        items
    }

    /// Formats the statistics as a human-readable summary string.
    #[must_use]
    pub fn summary(&self) -> String {
        use std::fmt::Write;

        let mut output = String::new();
        let _ = writeln!(output, "ADR Statistics");
        let _ = writeln!(output, "==============");
        let _ = writeln!(output, "Total: {} records", self.total_count);

        // Status breakdown
        let mut status_parts: Vec<String> = Vec::new();
        for status in Status::all() {
            let key = status.as_str().to_string();
            let count = self.by_status.get(&key).copied().unwrap_or(0);
            if count > 0 {
                status_parts.push(format!("{} ({})", status, count));
            }
        }
        if !status_parts.is_empty() {
            let _ = writeln!(output, "By Status: {}", status_parts.join(", "));
        }

        // Category breakdown (top 5)
        if !self.by_category.is_empty() {
            let top = Self::top_n(&self.by_category, 5);
            let parts: Vec<String> = top.iter().map(|(k, v)| format!("{k} ({v})")).collect();
            let _ = writeln!(output, "By Category: {}", parts.join(", "));
        }

        // Author breakdown (top 5)
        if !self.by_author.is_empty() {
            let top = Self::top_n(&self.by_author, 5);
            let parts: Vec<String> = top.iter().map(|(k, v)| format!("{k} ({v})")).collect();
            let _ = writeln!(output, "Authors: {}", parts.join(", "));
        }

        // Date range
        match (&self.earliest_date, &self.latest_date) {
            (Some(earliest), Some(latest)) => {
                let _ = writeln!(output, "Date Range: {earliest} -> {latest}");
            },
            _ => {},
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AdrId, Frontmatter};
    use std::path::PathBuf;
    use time::macros::date;

    fn create_test_adr(title: &str, status: Status, category: &str) -> Adr {
        let frontmatter = Frontmatter::new(title)
            .with_status(status)
            .with_category(category)
            .with_created(date!(2025 - 01 - 15));

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
    fn test_statistics_empty() {
        let stats = AdrStatistics::from_adrs(&[]);
        assert_eq!(stats.total_count, 0);
    }

    #[test]
    fn test_statistics_by_status() {
        let adrs = vec![
            create_test_adr("ADR 1", Status::Accepted, "arch"),
            create_test_adr("ADR 2", Status::Accepted, "api"),
            create_test_adr("ADR 3", Status::Proposed, "arch"),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.by_status.get("accepted"), Some(&2));
        assert_eq!(stats.by_status.get("proposed"), Some(&1));
    }

    #[test]
    fn test_statistics_by_category() {
        let adrs = vec![
            create_test_adr("ADR 1", Status::Accepted, "architecture"),
            create_test_adr("ADR 2", Status::Accepted, "architecture"),
            create_test_adr("ADR 3", Status::Proposed, "api"),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.by_category.get("architecture"), Some(&2));
        assert_eq!(stats.by_category.get("api"), Some(&1));
    }

    #[test]
    fn test_statistics_date_range() {
        let mut fm1 = Frontmatter::new("Early");
        fm1.created = Some(date!(2024 - 01 - 01));

        let mut fm2 = Frontmatter::new("Late");
        fm2.created = Some(date!(2025 - 06 - 15));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.earliest_date, Some(date!(2024 - 01 - 01)));
        assert_eq!(stats.latest_date, Some(date!(2025 - 06 - 15)));
    }

    #[test]
    fn test_top_n() {
        let mut counts = HashMap::new();
        counts.insert("a", 10);
        counts.insert("b", 5);
        counts.insert("c", 20);
        counts.insert("d", 1);

        let top = AdrStatistics::top_n(&counts, 2);

        assert_eq!(top.len(), 2);
        assert_eq!(top[0], ("c", 20));
        assert_eq!(top[1], ("a", 10));
    }

    #[test]
    fn test_summary_format() {
        let adrs = vec![create_test_adr(
            "Test ADR",
            Status::Accepted,
            "architecture",
        )];

        let stats = AdrStatistics::from_adrs(&adrs);
        let summary = stats.summary();

        assert!(summary.contains("ADR Statistics"));
        assert!(summary.contains("Total: 1"));
        assert!(summary.contains("accepted"));
    }

    #[test]
    fn test_statistics_by_author() {
        let fm1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_author("Alice")
            .with_created(date!(2025 - 01 - 15));

        let fm2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_author("Bob")
            .with_created(date!(2025 - 01 - 15));

        let fm3 = Frontmatter::new("ADR 3")
            .with_status(Status::Accepted)
            .with_author("Alice")
            .with_created(date!(2025 - 01 - 15));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("3"),
                "3.md".to_string(),
                PathBuf::from("3.md"),
                fm3,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.by_author.get("Alice"), Some(&2));
        assert_eq!(stats.by_author.get("Bob"), Some(&1));
    }

    #[test]
    fn test_statistics_by_technology() {
        let fm1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_technologies(vec!["rust".to_string(), "postgres".to_string()])
            .with_created(date!(2025 - 01 - 15));

        let fm2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_technologies(vec!["rust".to_string(), "redis".to_string()])
            .with_created(date!(2025 - 01 - 15));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.by_technology.get("rust"), Some(&2));
        assert_eq!(stats.by_technology.get("postgres"), Some(&1));
        assert_eq!(stats.by_technology.get("redis"), Some(&1));
    }

    #[test]
    fn test_statistics_by_project() {
        let fm1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_project("project-alpha")
            .with_created(date!(2025 - 01 - 15));

        let fm2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_project("project-beta")
            .with_created(date!(2025 - 01 - 15));

        let fm3 = Frontmatter::new("ADR 3")
            .with_status(Status::Accepted)
            .with_project("project-alpha")
            .with_created(date!(2025 - 01 - 15));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("3"),
                "3.md".to_string(),
                PathBuf::from("3.md"),
                fm3,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.by_project.get("project-alpha"), Some(&2));
        assert_eq!(stats.by_project.get("project-beta"), Some(&1));
    }

    #[test]
    fn test_statistics_by_tag() {
        let fm1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_tags(vec!["database".to_string(), "performance".to_string()])
            .with_created(date!(2025 - 01 - 15));

        let fm2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_tags(vec!["database".to_string(), "security".to_string()])
            .with_created(date!(2025 - 01 - 15));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);

        assert_eq!(stats.by_tag.get("database"), Some(&2));
        assert_eq!(stats.by_tag.get("performance"), Some(&1));
        assert_eq!(stats.by_tag.get("security"), Some(&1));
    }

    #[test]
    fn test_summary_with_all_fields() {
        let fm1 = Frontmatter::new("ADR 1")
            .with_status(Status::Accepted)
            .with_category("architecture")
            .with_author("Alice")
            .with_created(date!(2025 - 01 - 15));

        let fm2 = Frontmatter::new("ADR 2")
            .with_status(Status::Proposed)
            .with_category("api")
            .with_author("Bob")
            .with_created(date!(2025 - 06 - 20));

        let adrs = vec![
            Adr::new(
                AdrId::new("1"),
                "1.md".to_string(),
                PathBuf::from("1.md"),
                fm1,
                String::new(),
                String::new(),
                String::new(),
            ),
            Adr::new(
                AdrId::new("2"),
                "2.md".to_string(),
                PathBuf::from("2.md"),
                fm2,
                String::new(),
                String::new(),
                String::new(),
            ),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);
        let summary = stats.summary();

        assert!(summary.contains("Total: 2 records"));
        assert!(summary.contains("By Category:"));
        assert!(summary.contains("Authors:"));
        assert!(summary.contains("Date Range:"));
    }
}
