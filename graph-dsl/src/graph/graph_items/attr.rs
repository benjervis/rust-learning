use std::collections::HashMap;

pub fn attr_list_to_hash_map(attrs: &[(&str, &str)]) -> HashMap<String, String> {
    HashMap::from_iter(
        attrs
            .iter()
            .map(|(key, val)| (key.to_string(), val.to_string())),
    )
}

pub fn attr<'a>(attrs: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    attrs.get(key).map(|val| val.as_str())
}
