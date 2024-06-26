use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    id: String,
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
        self.attrs = HashMap::from_iter(
            attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );
        self
    }
}
