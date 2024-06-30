use std::collections::HashMap;

use super::attr::{attr, attr_list_to_hash_map};

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            attrs: HashMap::default(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attr_list_to_hash_map(attrs);
        self
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        attr(&self.attrs, key)
    }
}
