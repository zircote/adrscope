//! Wiki-style markdown generation.
//!
//! Generates markdown files suitable for GitHub Wiki.

use std::collections::HashMap;
use std::fmt::Write;

use crate::domain::{Adr, AdrStatistics, Status};
use crate::error::Result;

/// Renderer for wiki-style markdown output.
#[derive(Debug, Clone, Default)]
pub struct WikiRenderer;

impl WikiRenderer {
    /// Creates a new wiki renderer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates the main ADR index page.
    #[must_use]
    pub fn render_index(&self, adrs: &[Adr], pages_url: Option<&str>) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "# ADR Index");
        let _ = writeln!(output);

        if let Some(url) = pages_url {
            let _ = writeln!(output, "> [View Interactive ADRScope Viewer]({url})");
            let _ = writeln!(output);
        }

        let _ = writeln!(output, "| ID | Title | Status | Category | Created |");
        let _ = writeln!(output, "|:---|:------|:------:|:---------|:--------|");

        for adr in adrs {
            let created = adr
                .created()
                .map_or_else(|| "-".to_string(), |d| d.to_string());

            let status_badge = status_badge(adr.status());

            let _ = writeln!(
                output,
                "| {} | [{}]({}) | {} | {} | {} |",
                adr.id(),
                adr.title(),
                adr.filename(),
                status_badge,
                adr.category(),
                created
            );
        }

        output
    }

    /// Generates an ADR listing grouped by status.
    #[must_use]
    pub fn render_by_status(&self, adrs: &[Adr]) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "# ADRs by Status");
        let _ = writeln!(output);

        // Group ADRs by status
        let mut by_status: HashMap<Status, Vec<&Adr>> = HashMap::new();
        for adr in adrs {
            by_status.entry(adr.status()).or_default().push(adr);
        }

        // Output in a fixed order
        for status in Status::all() {
            if let Some(group) = by_status.get(status) {
                if !group.is_empty() {
                    let _ = writeln!(output, "## {} {}", status_emoji(*status), status);
                    let _ = writeln!(output);

                    for adr in group {
                        let _ = writeln!(
                            output,
                            "- [{}]({}) - {}",
                            adr.title(),
                            adr.filename(),
                            adr.description()
                        );
                    }
                    let _ = writeln!(output);
                }
            }
        }

        output
    }

    /// Generates an ADR listing grouped by category.
    #[must_use]
    pub fn render_by_category(&self, adrs: &[Adr]) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "# ADRs by Category");
        let _ = writeln!(output);

        // Group ADRs by category
        let mut by_category: HashMap<&str, Vec<&Adr>> = HashMap::new();
        for adr in adrs {
            let category = if adr.category().is_empty() {
                "Uncategorized"
            } else {
                adr.category()
            };
            by_category.entry(category).or_default().push(adr);
        }

        // Sort categories alphabetically
        let mut categories: Vec<_> = by_category.keys().collect();
        categories.sort();

        for category in categories {
            if let Some(group) = by_category.get(category) {
                let _ = writeln!(output, "## {category}");
                let _ = writeln!(output);

                for adr in group {
                    let status = status_badge(adr.status());
                    let _ = writeln!(
                        output,
                        "- [{}]({}) {} - {}",
                        adr.title(),
                        adr.filename(),
                        status,
                        truncate(adr.description(), 80)
                    );
                }
                let _ = writeln!(output);
            }
        }

        output
    }

    /// Generates a chronological timeline of ADRs.
    #[must_use]
    pub fn render_timeline(&self, adrs: &[Adr]) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "# ADR Timeline");
        let _ = writeln!(output);

        // Sort ADRs by created date (newest first)
        let mut sorted: Vec<&Adr> = adrs.iter().collect();
        sorted.sort_by(|a, b| b.created().cmp(&a.created()));

        // Group by year-month
        let mut current_month: Option<String> = None;

        for adr in &sorted {
            if let Some(date) = adr.created() {
                let month_key = format!("{}-{:02}", date.year(), date.month() as u8);

                if current_month.as_ref() != Some(&month_key) {
                    current_month = Some(month_key);
                    let _ = writeln!(output, "\n## {} {}", date.month(), date.year());
                    let _ = writeln!(output);
                }

                let status = status_badge(adr.status());
                let _ = writeln!(
                    output,
                    "- **{}** [{}]({}) {}",
                    date,
                    adr.title(),
                    adr.filename(),
                    status
                );
            }
        }

        // ADRs without dates
        let undated: Vec<_> = sorted.iter().filter(|a| a.created().is_none()).collect();
        if !undated.is_empty() {
            let _ = writeln!(output, "\n## Undated");
            let _ = writeln!(output);
            for adr in undated {
                let status = status_badge(adr.status());
                let _ = writeln!(output, "- [{}]({}) {}", adr.title(), adr.filename(), status);
            }
        }

        output
    }

    /// Generates a statistics summary page.
    #[must_use]
    pub fn render_statistics(&self, stats: &AdrStatistics) -> String {
        let mut output = String::new();

        let _ = writeln!(output, "# ADR Statistics");
        let _ = writeln!(output);
        let _ = writeln!(output, "**Total ADRs:** {}", stats.total_count);
        let _ = writeln!(output);

        // Status breakdown
        let _ = writeln!(output, "## By Status");
        let _ = writeln!(output);
        for status in Status::all() {
            let count = stats.by_status.get(status.as_str()).copied().unwrap_or(0);
            let _ = writeln!(output, "- {} {}: {}", status_emoji(*status), status, count);
        }
        let _ = writeln!(output);

        // Category breakdown
        if !stats.by_category.is_empty() {
            let _ = writeln!(output, "## By Category");
            let _ = writeln!(output);
            let mut categories: Vec<_> = stats.by_category.iter().collect();
            categories.sort_by(|a, b| b.1.cmp(a.1));
            for (category, count) in categories {
                let _ = writeln!(output, "- {category}: {count}");
            }
            let _ = writeln!(output);
        }

        // Author breakdown
        if !stats.by_author.is_empty() {
            let _ = writeln!(output, "## By Author");
            let _ = writeln!(output);
            let mut authors: Vec<_> = stats.by_author.iter().collect();
            authors.sort_by(|a, b| b.1.cmp(a.1));
            for (author, count) in authors.iter().take(10) {
                let _ = writeln!(output, "- {author}: {count}");
            }
            let _ = writeln!(output);
        }

        // Date range
        if let (Some(earliest), Some(latest)) = (&stats.earliest_date, &stats.latest_date) {
            let _ = writeln!(output, "## Date Range");
            let _ = writeln!(output);
            let _ = writeln!(output, "- **Earliest:** {earliest}");
            let _ = writeln!(output, "- **Latest:** {latest}");
        }

        output
    }

    /// Generates all wiki files.
    pub fn render_all(
        &self,
        adrs: &[Adr],
        pages_url: Option<&str>,
    ) -> Result<Vec<(String, String)>> {
        let stats = AdrStatistics::from_adrs(adrs);

        Ok(vec![
            (
                "ADR-Index.md".to_string(),
                self.render_index(adrs, pages_url),
            ),
            ("ADR-By-Status.md".to_string(), self.render_by_status(adrs)),
            (
                "ADR-By-Category.md".to_string(),
                self.render_by_category(adrs),
            ),
            ("ADR-Timeline.md".to_string(), self.render_timeline(adrs)),
            (
                "ADR-Statistics.md".to_string(),
                self.render_statistics(&stats),
            ),
        ])
    }
}

/// Returns an emoji for the given status.
fn status_emoji(status: Status) -> &'static str {
    match status {
        Status::Proposed => "\u{1F7E1}",   // yellow circle
        Status::Accepted => "\u{2705}",    // green check
        Status::Deprecated => "\u{1F534}", // red circle
        Status::Superseded => "\u{26AA}",  // white circle
    }
}

/// Returns a markdown badge for the given status.
fn status_badge(status: Status) -> String {
    format!("`{}`", status.as_str())
}

/// Truncates a string to the given length, adding ellipsis if needed.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AdrId, Frontmatter};
    use std::path::PathBuf;
    use time::macros::date;

    fn create_test_adr(id: &str, title: &str, status: Status, category: &str) -> Adr {
        let frontmatter = Frontmatter::new(title)
            .with_status(status)
            .with_category(category)
            .with_description(format!("Description for {title}"))
            .with_created(date!(2025 - 01 - 15));

        Adr::new(
            AdrId::new(id),
            format!("{id}.md"),
            PathBuf::from(format!("{id}.md")),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        )
    }

    #[test]
    fn test_render_index() {
        let adrs = vec![
            create_test_adr("adr_0001", "Use PostgreSQL", Status::Accepted, "database"),
            create_test_adr("adr_0002", "Use Rust", Status::Proposed, "language"),
        ];

        let renderer = WikiRenderer::new();
        let output = renderer.render_index(&adrs, Some("https://example.com/adrs"));

        assert!(output.contains("# ADR Index"));
        assert!(output.contains("[View Interactive ADRScope Viewer]"));
        assert!(output.contains("Use PostgreSQL"));
        assert!(output.contains("adr_0001.md"));
    }

    #[test]
    fn test_render_by_status() {
        let adrs = vec![
            create_test_adr("adr_0001", "ADR 1", Status::Accepted, "cat"),
            create_test_adr("adr_0002", "ADR 2", Status::Accepted, "cat"),
            create_test_adr("adr_0003", "ADR 3", Status::Proposed, "cat"),
        ];

        let renderer = WikiRenderer::new();
        let output = renderer.render_by_status(&adrs);

        assert!(output.contains("# ADRs by Status"));
        assert!(output.contains("## ")); // Status headers
    }

    #[test]
    fn test_render_by_category() {
        let adrs = vec![
            create_test_adr("adr_0001", "ADR 1", Status::Accepted, "architecture"),
            create_test_adr("adr_0002", "ADR 2", Status::Accepted, "api"),
        ];

        let renderer = WikiRenderer::new();
        let output = renderer.render_by_category(&adrs);

        assert!(output.contains("# ADRs by Category"));
        assert!(output.contains("## api"));
        assert!(output.contains("## architecture"));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a long string", 10), "this is...");
    }

    #[test]
    fn test_status_badge() {
        assert_eq!(status_badge(Status::Accepted), "`accepted`");
        assert_eq!(status_badge(Status::Proposed), "`proposed`");
    }

    #[test]
    fn test_status_emoji() {
        assert_eq!(status_emoji(Status::Proposed), "\u{1F7E1}");
        assert_eq!(status_emoji(Status::Accepted), "\u{2705}");
        assert_eq!(status_emoji(Status::Deprecated), "\u{1F534}");
        assert_eq!(status_emoji(Status::Superseded), "\u{26AA}");
    }

    #[test]
    fn test_render_timeline() {
        let adrs = vec![
            create_test_adr("adr_0001", "First ADR", Status::Accepted, "arch"),
            create_test_adr("adr_0002", "Second ADR", Status::Proposed, "api"),
        ];

        let renderer = WikiRenderer::new();
        let output = renderer.render_timeline(&adrs);

        assert!(output.contains("# ADR Timeline"));
        assert!(output.contains("2025"));
        assert!(output.contains("First ADR"));
        assert!(output.contains("Second ADR"));
    }

    #[test]
    fn test_render_timeline_with_undated() {
        // Create an ADR without a date
        let frontmatter = Frontmatter::new("Undated ADR")
            .with_status(Status::Proposed)
            .with_category("test");

        let undated_adr = Adr::new(
            AdrId::new("adr_undated"),
            "adr_undated.md".to_string(),
            PathBuf::from("adr_undated.md"),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        );

        let adrs = vec![
            create_test_adr("adr_0001", "Dated ADR", Status::Accepted, "arch"),
            undated_adr,
        ];

        let renderer = WikiRenderer::new();
        let output = renderer.render_timeline(&adrs);

        assert!(output.contains("# ADR Timeline"));
        assert!(output.contains("## Undated"));
        assert!(output.contains("Undated ADR"));
    }

    #[test]
    fn test_render_statistics() {
        let adrs = vec![
            create_test_adr("adr_0001", "ADR 1", Status::Accepted, "arch"),
            create_test_adr("adr_0002", "ADR 2", Status::Accepted, "api"),
            create_test_adr("adr_0003", "ADR 3", Status::Proposed, "arch"),
        ];

        let stats = AdrStatistics::from_adrs(&adrs);
        let renderer = WikiRenderer::new();
        let output = renderer.render_statistics(&stats);

        assert!(output.contains("# ADR Statistics"));
        assert!(output.contains("**Total ADRs:** 3"));
        assert!(output.contains("## By Status"));
        assert!(output.contains("## By Category"));
    }

    #[test]
    fn test_render_statistics_with_authors() {
        let frontmatter = Frontmatter::new("ADR with Author")
            .with_status(Status::Accepted)
            .with_category("arch")
            .with_author("Test Author")
            .with_created(date!(2025 - 01 - 15));

        let adr = Adr::new(
            AdrId::new("adr_0001"),
            "adr_0001.md".to_string(),
            PathBuf::from("adr_0001.md"),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        );

        let stats = AdrStatistics::from_adrs(&[adr]);
        let renderer = WikiRenderer::new();
        let output = renderer.render_statistics(&stats);

        assert!(output.contains("## By Author"));
        assert!(output.contains("Test Author"));
    }

    #[test]
    fn test_render_all() {
        let adrs = vec![
            create_test_adr("adr_0001", "ADR 1", Status::Accepted, "arch"),
            create_test_adr("adr_0002", "ADR 2", Status::Proposed, "api"),
        ];

        let renderer = WikiRenderer::new();
        let files = renderer
            .render_all(&adrs, Some("https://example.com"))
            .expect("should render all");

        assert_eq!(files.len(), 5);

        let filenames: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
        assert!(filenames.contains(&"ADR-Index.md"));
        assert!(filenames.contains(&"ADR-By-Status.md"));
        assert!(filenames.contains(&"ADR-By-Category.md"));
        assert!(filenames.contains(&"ADR-Timeline.md"));
        assert!(filenames.contains(&"ADR-Statistics.md"));
    }

    #[test]
    fn test_render_index_without_url() {
        let adrs = vec![create_test_adr(
            "adr_0001",
            "Test ADR",
            Status::Accepted,
            "test",
        )];

        let renderer = WikiRenderer::new();
        let output = renderer.render_index(&adrs, None);

        assert!(output.contains("# ADR Index"));
        assert!(!output.contains("[View Interactive ADRScope Viewer]"));
    }

    #[test]
    fn test_render_by_category_uncategorized() {
        // Create an ADR without a category
        let frontmatter = Frontmatter::new("Uncategorized ADR")
            .with_status(Status::Proposed)
            .with_created(date!(2025 - 01 - 15));

        let uncategorized_adr = Adr::new(
            AdrId::new("adr_uncat"),
            "adr_uncat.md".to_string(),
            PathBuf::from("adr_uncat.md"),
            frontmatter,
            String::new(),
            String::new(),
            String::new(),
        );

        let adrs = vec![uncategorized_adr];

        let renderer = WikiRenderer::new();
        let output = renderer.render_by_category(&adrs);

        assert!(output.contains("## Uncategorized"));
    }

    #[test]
    fn test_truncate_edge_cases() {
        // Exactly at max length
        assert_eq!(truncate("12345678", 8), "12345678");
        // Just over max length
        assert_eq!(truncate("123456789", 8), "12345...");
        // Empty string
        assert_eq!(truncate("", 10), "");
        // Very short max
        assert_eq!(truncate("hello", 3), "...");
    }
}
