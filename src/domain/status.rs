//! ADR status lifecycle states.
//!
//! Status represents the lifecycle state of an Architecture Decision Record,
//! following the structured-madr specification.

use serde::{Deserialize, Serialize};

/// The status of an Architecture Decision Record in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The decision is under discussion and not yet finalized.
    #[default]
    Proposed,
    /// The decision has been accepted and is in effect.
    Accepted,
    /// The decision has been deprecated and should not be used for new work.
    Deprecated,
    /// The decision has been replaced by a newer decision.
    Superseded,
}

impl Status {
    /// Returns the status as a lowercase string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Accepted => "accepted",
            Self::Deprecated => "deprecated",
            Self::Superseded => "superseded",
        }
    }

    /// Returns the CSS class name for styling this status.
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Proposed => "status-proposed",
            Self::Accepted => "status-accepted",
            Self::Deprecated => "status-deprecated",
            Self::Superseded => "status-superseded",
        }
    }

    /// Returns the display color for this status in hex format.
    #[must_use]
    pub const fn color(&self) -> &'static str {
        match self {
            Self::Proposed => "#f59e0b",   // amber
            Self::Accepted => "#10b981",   // green
            Self::Deprecated => "#ef4444", // red
            Self::Superseded => "#6b7280", // gray
        }
    }

    /// Returns all possible status values.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Proposed,
            Self::Accepted,
            Self::Deprecated,
            Self::Superseded,
        ]
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(Self::Proposed),
            "accepted" => Ok(Self::Accepted),
            "deprecated" => Ok(Self::Deprecated),
            "superseded" => Ok(Self::Superseded),
            _ => Err(format!("invalid status: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_as_str() {
        assert_eq!(Status::Proposed.as_str(), "proposed");
        assert_eq!(Status::Accepted.as_str(), "accepted");
        assert_eq!(Status::Deprecated.as_str(), "deprecated");
        assert_eq!(Status::Superseded.as_str(), "superseded");
    }

    #[test]
    fn test_status_default() {
        assert_eq!(Status::default(), Status::Proposed);
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!("proposed".parse::<Status>().ok(), Some(Status::Proposed));
        assert_eq!("ACCEPTED".parse::<Status>().ok(), Some(Status::Accepted));
        assert_eq!(
            "Deprecated".parse::<Status>().ok(),
            Some(Status::Deprecated)
        );
        assert!("invalid".parse::<Status>().is_err());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Accepted), "accepted");
    }

    #[test]
    fn test_status_css_class() {
        assert_eq!(Status::Proposed.css_class(), "status-proposed");
        assert_eq!(Status::Accepted.css_class(), "status-accepted");
        assert_eq!(Status::Deprecated.css_class(), "status-deprecated");
        assert_eq!(Status::Superseded.css_class(), "status-superseded");
    }

    #[test]
    fn test_status_color() {
        assert_eq!(Status::Proposed.color(), "#f59e0b");
        assert_eq!(Status::Accepted.color(), "#10b981");
        assert_eq!(Status::Deprecated.color(), "#ef4444");
        assert_eq!(Status::Superseded.color(), "#6b7280");
    }

    #[test]
    fn test_status_all() {
        let all = Status::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&Status::Proposed));
        assert!(all.contains(&Status::Accepted));
        assert!(all.contains(&Status::Deprecated));
        assert!(all.contains(&Status::Superseded));
    }

    #[test]
    fn test_status_serialization() {
        let status = Status::Accepted;
        let json = serde_json::to_string(&status).ok();
        assert_eq!(json, Some("\"accepted\"".to_string()));
    }

    #[test]
    fn test_status_deserialization() {
        let status: Status = serde_json::from_str("\"proposed\"").expect("should parse");
        assert_eq!(status, Status::Proposed);
    }
}
