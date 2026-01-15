//! Graph data structures for ADR relationships.
//!
//! This module provides types for representing ADR relationships as a graph,
//! enabling network visualization of related decisions.

use serde::Serialize;

use super::{Adr, Status};

/// A node in the ADR relationship graph.
#[derive(Debug, Clone, Serialize)]
pub struct Node {
    /// The ADR identifier.
    pub id: String,
    /// The ADR status for coloring.
    pub status: String,
    /// The ADR title for display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Node {
    /// Creates a new node from an ADR.
    #[must_use]
    pub fn from_adr(adr: &Adr) -> Self {
        Self {
            id: adr.id().as_str().to_string(),
            status: adr.status().as_str().to_string(),
            title: Some(adr.title().to_string()),
        }
    }

    /// Creates a new node with just an ID (for referenced but non-existent ADRs).
    #[must_use]
    pub fn placeholder(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: Status::default().as_str().to_string(),
            title: None,
        }
    }
}

/// The type of relationship between two ADRs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeType {
    /// A general relationship (from `related` field).
    Related,
    /// One ADR supersedes another.
    Supersedes,
}

impl EdgeType {
    /// Returns the edge type as a string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Related => "related",
            Self::Supersedes => "supersedes",
        }
    }
}

/// An edge connecting two ADRs in the graph.
#[derive(Debug, Clone, Serialize)]
pub struct Edge {
    /// Source ADR identifier.
    pub source: String,
    /// Target ADR identifier.
    pub target: String,
    /// Type of relationship.
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
}

impl Edge {
    /// Creates a new edge.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>, edge_type: EdgeType) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            edge_type,
        }
    }

    /// Creates a "related" edge.
    #[must_use]
    pub fn related(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Related)
    }

    /// Creates a "supersedes" edge.
    #[must_use]
    pub fn supersedes(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Supersedes)
    }
}

/// The complete ADR relationship graph.
#[derive(Debug, Clone, Serialize)]
pub struct Graph {
    /// All nodes (ADRs) in the graph.
    pub nodes: Vec<Node>,
    /// All edges (relationships) between ADRs.
    pub edges: Vec<Edge>,
}

impl Graph {
    /// Creates a new empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Builds a graph from a collection of ADRs.
    #[must_use]
    pub fn from_adrs(adrs: &[Adr]) -> Self {
        let mut nodes: Vec<Node> = adrs.iter().map(Node::from_adr).collect();
        let mut edges: Vec<Edge> = Vec::new();

        // Build a set of known ADR IDs for resolving references
        let known_ids: std::collections::HashSet<&str> =
            adrs.iter().map(|a| a.id().as_str()).collect();

        // Process relationships
        for adr in adrs {
            let source_id = adr.id().as_str();

            // Handle `related` references
            for related_ref in adr.related() {
                // Extract ID from filename reference (e.g., "adr_0005.md" -> "adr_0005")
                let target_id = extract_id_from_ref(related_ref);

                // Add edge
                edges.push(Edge::related(source_id, &target_id));

                // If target doesn't exist in our collection, add a placeholder node
                if !known_ids.contains(target_id.as_str()) {
                    nodes.push(Node::placeholder(&target_id));
                }
            }
        }

        // Remove duplicate nodes (placeholders for ADRs we later found)
        nodes.dedup_by(|a, b| a.id == b.id);

        Self { nodes, edges }
    }

    /// Returns the number of nodes in the graph.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the graph.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Checks if the graph is empty (no nodes).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts an ADR ID from a reference string.
///
/// Handles formats like "adr_0005.md" or just "adr_0005".
fn extract_id_from_ref(reference: &str) -> String {
    reference
        .strip_suffix(".md")
        .unwrap_or(reference)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AdrId, Frontmatter};
    use std::path::PathBuf;

    fn create_test_adr(id: &str, related: Vec<String>) -> Adr {
        let frontmatter = Frontmatter::new(format!("Test {id}")).with_related(related);
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
    fn test_node_from_adr() {
        let adr = create_test_adr("adr_0001", vec![]);
        let node = Node::from_adr(&adr);

        assert_eq!(node.id, "adr_0001");
        assert_eq!(node.status, "proposed");
        assert_eq!(node.title, Some("Test adr_0001".to_string()));
    }

    #[test]
    fn test_node_placeholder() {
        let node = Node::placeholder("adr_0005");
        assert_eq!(node.id, "adr_0005");
        assert!(node.title.is_none());
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::related("adr_0001", "adr_0005");
        assert_eq!(edge.source, "adr_0001");
        assert_eq!(edge.target, "adr_0005");
        assert_eq!(edge.edge_type, EdgeType::Related);
    }

    #[test]
    fn test_graph_from_adrs() {
        let adrs = vec![
            create_test_adr("adr_0001", vec!["adr_0002.md".to_string()]),
            create_test_adr("adr_0002", vec![]),
        ];

        let graph = Graph::from_adrs(&adrs);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.edges[0].source, "adr_0001");
        assert_eq!(graph.edges[0].target, "adr_0002");
    }

    #[test]
    fn test_graph_with_missing_reference() {
        let adrs = vec![create_test_adr(
            "adr_0001",
            vec!["adr_missing.md".to_string()],
        )];

        let graph = Graph::from_adrs(&adrs);

        // Should have 2 nodes: the actual ADR and a placeholder for the missing one
        assert_eq!(graph.node_count(), 2);
        assert!(graph.nodes.iter().any(|n| n.id == "adr_missing"));
    }

    #[test]
    fn test_extract_id_from_ref() {
        assert_eq!(extract_id_from_ref("adr_0005.md"), "adr_0005");
        assert_eq!(extract_id_from_ref("adr_0005"), "adr_0005");
    }

    #[test]
    fn test_edge_type_as_str() {
        assert_eq!(EdgeType::Related.as_str(), "related");
        assert_eq!(EdgeType::Supersedes.as_str(), "supersedes");
    }

    #[test]
    fn test_edge_supersedes() {
        let edge = Edge::supersedes("adr_0002", "adr_0001");
        assert_eq!(edge.source, "adr_0002");
        assert_eq!(edge.target, "adr_0001");
        assert_eq!(edge.edge_type, EdgeType::Supersedes);
    }

    #[test]
    fn test_graph_new() {
        let graph = Graph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_default() {
        let graph = Graph::default();
        assert!(graph.is_empty());
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_graph_is_empty() {
        let empty_graph = Graph::new();
        assert!(empty_graph.is_empty());

        let non_empty_graph = Graph::from_adrs(&[create_test_adr("adr_0001", vec![])]);
        assert!(!non_empty_graph.is_empty());
    }

    #[test]
    fn test_edge_new() {
        let edge = Edge::new("source", "target", EdgeType::Related);
        assert_eq!(edge.source, "source");
        assert_eq!(edge.target, "target");
        assert_eq!(edge.edge_type, EdgeType::Related);
    }

    #[test]
    fn test_node_serialization() {
        let node = Node::placeholder("adr_0001");
        let json = serde_json::to_string(&node).expect("should serialize");
        assert!(json.contains("\"id\":\"adr_0001\""));
        // title should be skipped when None
        assert!(!json.contains("\"title\":null"));
    }

    #[test]
    fn test_edge_serialization() {
        let edge = Edge::supersedes("adr_0002", "adr_0001");
        let json = serde_json::to_string(&edge).expect("should serialize");
        assert!(json.contains("\"source\":\"adr_0002\""));
        assert!(json.contains("\"target\":\"adr_0001\""));
        assert!(json.contains("\"type\":\"supersedes\""));
    }

    #[test]
    fn test_graph_serialization() {
        let adrs = vec![
            create_test_adr("adr_0001", vec!["adr_0002.md".to_string()]),
            create_test_adr("adr_0002", vec![]),
        ];
        let graph = Graph::from_adrs(&adrs);
        let json = serde_json::to_string(&graph).expect("should serialize");
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
    }
}
