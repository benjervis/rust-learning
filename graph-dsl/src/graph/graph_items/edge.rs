use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    pub attrs: HashMap<String, String>,
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
        self.attrs = HashMap::from_iter(
            attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );
        self
    }
}
