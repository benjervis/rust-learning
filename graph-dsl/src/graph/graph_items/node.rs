use std::collections::HashMap;

use super::attr::{attr, attr_list_to_hash_map};

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
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
