//! Domain layer containing core business logic.
//!
//! This module contains the core domain types and logic for ADRScope,
//! independent of external concerns like I/O, parsing, or rendering.

mod adr;
mod facets;
mod frontmatter;
mod graph;
mod stats;
mod status;
mod validation;

pub use adr::{Adr, AdrId};
pub use facets::{Facet, FacetValue, Facets};
pub use frontmatter::Frontmatter;
pub use graph::{Edge, EdgeType, Graph, Node};
pub use stats::AdrStatistics;
pub use status::Status;
pub use validation::{
    RecommendedFieldsRule, RequiredFieldsRule, Severity, ValidationIssue, ValidationReport,
    ValidationRule, Validator, default_rules,
};
