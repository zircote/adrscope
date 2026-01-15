//! HTML viewer generation using askama templates.

use askama::Template;
use serde::Serialize;
use time::OffsetDateTime;

use crate::domain::{Adr, Facets, Graph};
use crate::error::{Error, Result};

/// Theme for the HTML viewer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Theme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
    /// Auto (follows system preference).
    #[default]
    Auto,
}

impl Theme {
    /// Returns the theme as a string for use in templates.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Auto => "auto",
        }
    }
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "auto" => Ok(Self::Auto),
            _ => Err(format!("invalid theme: {s}")),
        }
    }
}

/// Configuration for HTML rendering.
#[derive(Debug, Clone, Default)]
pub struct RenderConfig {
    /// Page title.
    pub title: String,
    /// Theme preference.
    pub theme: Theme,
    /// Whether to embed all assets inline.
    pub embed_assets: bool,
}

impl RenderConfig {
    /// Creates a new render configuration with the given title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            theme: Theme::default(),
            embed_assets: true,
        }
    }

    /// Sets the theme.
    #[must_use]
    pub const fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

/// Data structure embedded in the HTML for JavaScript consumption.
#[derive(Debug, Clone, Serialize)]
pub struct ViewerData {
    /// Metadata about the generation.
    pub meta: ViewerMeta,
    /// All parsed ADRs.
    pub records: Vec<Adr>,
    /// Faceted filter data.
    pub facets: Facets,
    /// Relationship graph.
    pub graph: Graph,
}

/// Metadata embedded in the viewer.
#[derive(Debug, Clone, Serialize)]
pub struct ViewerMeta {
    /// When the viewer was generated.
    pub generated: String,
    /// Generator name and version.
    pub generator: String,
    /// Schema version.
    pub schema_version: String,
    /// Source directory.
    pub source_dir: String,
}

impl ViewerMeta {
    /// Creates metadata for the current generation.
    #[must_use]
    pub fn new(source_dir: impl Into<String>) -> Self {
        Self {
            generated: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "unknown".to_string()),
            generator: format!("adrscope/{}", env!("CARGO_PKG_VERSION")),
            schema_version: "1.0.0".to_string(),
            source_dir: source_dir.into(),
        }
    }
}

/// The main HTML viewer template.
#[derive(Template)]
#[template(path = "viewer.html", escape = "none")]
pub struct ViewerTemplate<'a> {
    /// Page title.
    pub title: &'a str,
    /// Theme preference.
    pub theme: &'a str,
    /// Serialized JSON data for embedding.
    pub data_json: &'a str,
    /// Embedded CSS.
    pub css: &'a str,
    /// Embedded JavaScript.
    pub js: &'a str,
}

/// HTML renderer for generating self-contained viewers.
#[derive(Debug, Clone, Default)]
pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Creates a new HTML renderer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Renders a collection of ADRs to a self-contained HTML viewer.
    pub fn render(
        &self,
        adrs: Vec<Adr>,
        source_dir: &str,
        config: &RenderConfig,
    ) -> Result<String> {
        // Build the embedded data
        let data = ViewerData {
            meta: ViewerMeta::new(source_dir),
            facets: Facets::from_adrs(&adrs),
            graph: Graph::from_adrs(&adrs),
            records: adrs,
        };

        // Serialize to JSON
        let data_json =
            serde_json::to_string(&data).map_err(|e| Error::JsonSerialize(e.to_string()))?;

        // Render the template
        let template = ViewerTemplate {
            title: &config.title,
            theme: config.theme.as_str(),
            data_json: &data_json,
            css: include_str!("../../../templates/styles.css"),
            js: include_str!("../../../templates/app.js"),
        };

        template.render().map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_from_str() {
        assert_eq!("light".parse::<Theme>().ok(), Some(Theme::Light));
        assert_eq!("DARK".parse::<Theme>().ok(), Some(Theme::Dark));
        assert_eq!("Auto".parse::<Theme>().ok(), Some(Theme::Auto));
        assert!("invalid".parse::<Theme>().is_err());
    }

    #[test]
    fn test_theme_as_str() {
        assert_eq!(Theme::Light.as_str(), "light");
        assert_eq!(Theme::Dark.as_str(), "dark");
        assert_eq!(Theme::Auto.as_str(), "auto");
    }

    #[test]
    fn test_render_config_builder() {
        let config = RenderConfig::new("My ADRs").with_theme(Theme::Dark);

        assert_eq!(config.title, "My ADRs");
        assert_eq!(config.theme, Theme::Dark);
    }

    #[test]
    fn test_viewer_meta_creation() {
        let meta = ViewerMeta::new("docs/decisions");

        assert!(meta.generated.contains("T")); // ISO 8601 format
        assert!(meta.generator.starts_with("adrscope/"));
        assert_eq!(meta.schema_version, "1.0.0");
        assert_eq!(meta.source_dir, "docs/decisions");
    }
}
