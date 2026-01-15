//! Unified error types for ADRScope operations.
//!
//! This module provides a single error enum that covers all failure modes
//! across the application, with rich context for debugging.

use std::path::PathBuf;
use thiserror::Error;

/// Error type for all ADRScope operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to read an ADR file from the filesystem.
    #[error("failed to read ADR file at {path}")]
    FileRead {
        /// Path to the file that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to write output to the filesystem.
    #[error("failed to write output to {path}")]
    FileWrite {
        /// Path where the write failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Invalid YAML frontmatter in an ADR file.
    #[error("invalid frontmatter in {path}: {message}")]
    InvalidFrontmatter {
        /// Path to the file with invalid frontmatter.
        path: PathBuf,
        /// Description of what's wrong.
        message: String,
    },

    /// YAML parsing failed.
    #[error("YAML parsing failed in {path}")]
    YamlParse {
        /// Path to the file that failed to parse.
        path: PathBuf,
        /// The underlying YAML error.
        #[source]
        source: serde_yaml::Error,
    },

    /// Missing required frontmatter field.
    #[error("missing required field '{field}' in {path}")]
    MissingField {
        /// Path to the file missing the field.
        path: PathBuf,
        /// Name of the missing field.
        field: &'static str,
    },

    /// Template rendering failed.
    #[error("template rendering failed")]
    TemplateRender {
        /// The underlying askama error.
        #[source]
        source: askama::Error,
    },

    /// No ADR files found in the specified directory.
    #[error("no ADR files found in {path}")]
    NoAdrsFound {
        /// Directory that was searched.
        path: PathBuf,
    },

    /// Validation failed with one or more errors.
    #[error("validation failed: {0} error(s) found")]
    ValidationFailed(usize),

    /// Invalid ADR filename format.
    #[error("invalid ADR filename: {0}")]
    InvalidFilename(String),

    /// Glob pattern error.
    #[error("invalid glob pattern: {0}")]
    GlobPattern(String),

    /// Date parsing error.
    #[error("invalid date format in {path}: {message}")]
    DateParse {
        /// Path to the file with the invalid date.
        path: PathBuf,
        /// Description of the date format issue.
        message: String,
    },

    /// JSON serialization error.
    #[error("JSON serialization failed: {0}")]
    JsonSerialize(String),
}

impl From<askama::Error> for Error {
    fn from(source: askama::Error) -> Self {
        Self::TemplateRender { source }
    }
}

/// Result type alias for ADRScope operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_file_read() {
        let err = Error::FileRead {
            path: PathBuf::from("/test/path.md"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        };
        let display = err.to_string();
        assert!(display.contains("failed to read ADR file"));
        assert!(display.contains("/test/path.md"));
    }

    #[test]
    fn test_error_display_missing_field() {
        let err = Error::MissingField {
            path: PathBuf::from("adr_0001.md"),
            field: "title",
        };
        let display = err.to_string();
        assert!(display.contains("missing required field"));
        assert!(display.contains("title"));
        assert!(display.contains("adr_0001.md"));
    }

    #[test]
    fn test_error_display_validation_failed() {
        let err = Error::ValidationFailed(5);
        assert_eq!(err.to_string(), "validation failed: 5 error(s) found");
    }

    #[test]
    fn test_error_display_no_adrs_found() {
        let err = Error::NoAdrsFound {
            path: PathBuf::from("docs/decisions"),
        };
        let display = err.to_string();
        assert!(display.contains("no ADR files found"));
        assert!(display.contains("docs/decisions"));
    }

    #[test]
    fn test_error_display_file_write() {
        let err = Error::FileWrite {
            path: PathBuf::from("/output/file.html"),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied"),
        };
        let display = err.to_string();
        assert!(display.contains("failed to write output"));
        assert!(display.contains("/output/file.html"));
    }

    #[test]
    fn test_error_display_invalid_frontmatter() {
        let err = Error::InvalidFrontmatter {
            path: PathBuf::from("test.md"),
            message: "missing closing delimiter".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("invalid frontmatter"));
        assert!(display.contains("test.md"));
        assert!(display.contains("missing closing delimiter"));
    }

    #[test]
    fn test_error_display_invalid_filename() {
        let err = Error::InvalidFilename("bad_name".to_string());
        let display = err.to_string();
        assert!(display.contains("invalid ADR filename"));
        assert!(display.contains("bad_name"));
    }

    #[test]
    fn test_error_display_glob_pattern() {
        let err = Error::GlobPattern("invalid pattern".to_string());
        let display = err.to_string();
        assert!(display.contains("invalid glob pattern"));
    }

    #[test]
    fn test_error_display_date_parse() {
        let err = Error::DateParse {
            path: PathBuf::from("adr.md"),
            message: "invalid date format".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("invalid date format"));
        assert!(display.contains("adr.md"));
    }

    #[test]
    fn test_error_display_json_serialize() {
        let err = Error::JsonSerialize("serialization failed".to_string());
        let display = err.to_string();
        assert!(display.contains("JSON serialization failed"));
    }

    #[test]
    fn test_error_from_askama() {
        // Create an askama error and convert it
        let askama_err = askama::Error::Custom(Box::new(std::io::Error::other("template error")));
        let err: Error = askama_err.into();
        let display = err.to_string();
        assert!(display.contains("template rendering failed"));
    }
}
