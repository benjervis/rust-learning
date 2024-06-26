use std::collections::HashMap;

use graph_items::{edge::Edge, node::Node};

pub mod graph_items;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

#[allow(clippy::new_without_default)]
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(&mut self, nodes: &Vec<Node>) -> &mut Self {
        self.nodes.clone_from(nodes);
        self
    }

    pub fn with_edges(&mut self, edges: &Vec<Edge>) -> &mut Self {
        self.edges.clone_from(edges);
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = HashMap::from_iter(
            attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );
        self
    }
}
