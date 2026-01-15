//! Command-line argument definitions using clap derive.

use clap::{Parser, Subcommand, ValueEnum};

/// ADRScope - Generate self-contained HTML viewers for Architecture Decision Records.
#[derive(Parser, Debug)]
#[command(name = "adrscope")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// The command to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a self-contained HTML viewer.
    Generate(GenerateArgs),

    /// Generate GitHub Wiki pages.
    Wiki(WikiArgs),

    /// Validate ADR files.
    Validate(ValidateArgs),

    /// Show ADR statistics.
    Stats(StatsArgs),
}

/// Arguments for the generate command.
#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Input directory containing ADR files.
    #[arg(short, long, default_value = "docs/decisions")]
    pub input: String,

    /// Output HTML file path.
    #[arg(short, long, default_value = "adrs.html")]
    pub output: String,

    /// Page title.
    #[arg(short, long, default_value = "Architecture Decision Records")]
    pub title: String,

    /// Theme preference.
    #[arg(long, value_enum, default_value = "auto")]
    pub theme: ThemeArg,

    /// Glob pattern for matching ADR files.
    #[arg(short, long, default_value = "**/*.md")]
    pub pattern: String,
}

/// Arguments for the wiki command.
#[derive(Parser, Debug)]
pub struct WikiArgs {
    /// Input directory containing ADR files.
    #[arg(short, long, default_value = "docs/decisions")]
    pub input: String,

    /// Output directory for wiki files.
    #[arg(short, long, default_value = "wiki")]
    pub output: String,

    /// URL to the GitHub Pages viewer (for cross-linking).
    #[arg(long)]
    pub pages_url: Option<String>,

    /// Glob pattern for matching ADR files.
    #[arg(short, long, default_value = "**/*.md")]
    pub pattern: String,
}

/// Arguments for the validate command.
#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// Input directory containing ADR files.
    #[arg(short, long, default_value = "docs/decisions")]
    pub input: String,

    /// Glob pattern for matching ADR files.
    #[arg(short, long, default_value = "**/*.md")]
    pub pattern: String,

    /// Fail on warnings (strict mode).
    #[arg(long)]
    pub strict: bool,
}

/// Arguments for the stats command.
#[derive(Parser, Debug)]
pub struct StatsArgs {
    /// Input directory containing ADR files.
    #[arg(short, long, default_value = "docs/decisions")]
    pub input: String,

    /// Glob pattern for matching ADR files.
    #[arg(short, long, default_value = "**/*.md")]
    pub pattern: String,

    /// Output format.
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: FormatArg,
}

/// Theme argument for CLI.
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ThemeArg {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
    /// Auto (follows system preference).
    #[default]
    Auto,
}

impl From<ThemeArg> for crate::infrastructure::Theme {
    fn from(arg: ThemeArg) -> Self {
        match arg {
            ThemeArg::Light => Self::Light,
            ThemeArg::Dark => Self::Dark,
            ThemeArg::Auto => Self::Auto,
        }
    }
}

/// Output format argument for CLI.
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum FormatArg {
    /// Human-readable text.
    #[default]
    Text,
    /// JSON format.
    Json,
    /// Markdown format.
    Markdown,
}

impl From<FormatArg> for crate::application::stats::StatsFormat {
    fn from(arg: FormatArg) -> Self {
        match arg {
            FormatArg::Text => Self::Text,
            FormatArg::Json => Self::Json,
            FormatArg::Markdown => Self::Markdown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        // Verify that the CLI structure is valid
        Cli::command().debug_assert();
    }

    #[test]
    fn test_generate_defaults() {
        let args = GenerateArgs {
            input: "docs/decisions".to_string(),
            output: "adrs.html".to_string(),
            title: "ADRs".to_string(),
            theme: ThemeArg::Auto,
            pattern: "**/*.md".to_string(),
        };

        assert_eq!(args.input, "docs/decisions");
        assert_eq!(args.output, "adrs.html");
    }

    #[test]
    fn test_theme_conversion() {
        use crate::infrastructure::Theme;

        assert!(matches!(Theme::from(ThemeArg::Light), Theme::Light));
        assert!(matches!(Theme::from(ThemeArg::Dark), Theme::Dark));
        assert!(matches!(Theme::from(ThemeArg::Auto), Theme::Auto));
    }

    #[test]
    fn test_format_conversion() {
        use crate::application::stats::StatsFormat;

        assert!(matches!(
            StatsFormat::from(FormatArg::Text),
            StatsFormat::Text
        ));
        assert!(matches!(
            StatsFormat::from(FormatArg::Json),
            StatsFormat::Json
        ));
        assert!(matches!(
            StatsFormat::from(FormatArg::Markdown),
            StatsFormat::Markdown
        ));
    }
}
