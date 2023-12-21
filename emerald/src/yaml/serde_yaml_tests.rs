/// This module contains tests for serializing and deserializing data using `serde_yaml`.
/// It demonstrates the conversion of various Rust data structures to and from YAML format.
#[cfg(test)]
mod tests {
    use serde_yaml;
    use std::collections::{BTreeMap, HashMap}; // 0.8.7

    /// Tests the serialization of a `BTreeMap` to a YAML string.
    /// A `BTreeMap` containing coordinates is serialized, and the output is asserted.
    #[test]
    fn serialize_btreemap_to_yaml() {
        // You have some type.
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), 1.0);
        map.insert("y".to_string(), 2.0);

        // Serialize it to a YAML string.
        let yaml = serde_yaml::to_string(&map).unwrap();
        assert_eq!(yaml, "x: 1.0\ny: 2.0\n");
    }

    /// Tests deserialization from a YAML string to a `BTreeMap`.
    /// A YAML string representing coordinates is deserialized and compared to an expected `BTreeMap`.
    #[test]
    fn deserialize_yaml_to_btreemap() {
        // You have some type.
        let yaml = "x: 1.0\ny: 2.0\n";

        // Deserialize it back to a Rust type.
        let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&yaml).unwrap();

        let mut map = BTreeMap::new();
        map.insert("x".to_string(), 1.0);
        map.insert("y".to_string(), 2.0);
        assert_eq!(map, deserialized_map);
    }

    /// Tests deserialization of YAML frontmatter typically contained in a markdown file to a `HashMap`.
    /// This test demonstrates converting a YAML string representing frontmatter into a `HashMap`.
    #[test]
    fn deserialize_yaml_frontmatter_to_hashmap() {
        // You have some type.
        let yaml = "tags: \"#Typ/Notiz\"\naliases: \ncreated: 2023-09-29T20:18\nmodified: 2023-10-01T20:16\nkeywords: ";

        // Deserialize it back to a Rust type.
        let deserialized_map: HashMap<String, String> = serde_yaml::from_str(&yaml).unwrap();

        let mut map = HashMap::new();
        map.insert("tags".to_string(), "#Typ/Notiz".to_string());
        map.insert("aliases".to_string(), "".to_string());
        map.insert("created".to_string(), "2023-09-29T20:18".to_string());
        map.insert("modified".to_string(), "2023-10-01T20:16".to_string());
        map.insert("keywords".to_string(), "".to_string());
        assert_eq!(map, deserialized_map);
    }

    /// Tests error handling for YAML frontmatter with document separators.
    /// Verifies that deserializing YAML containing dcoument separators results in an error,
    /// ensuring correct parser behavior for such cases.
    #[test]
    fn deserialize_yaml_frontmatter_with_dashes_returns_error() {
        // You have some type.
        let yaml = "---\ntags: \"#Typ/Notiz\"\naliases: \ncreated: 2023-09-29T20:18\nmodified: 2023-10-01T20:16\nkeywords: \n---";

        // Deserialize it back to a Rust type.
        let res: Result<HashMap<String, String>, _> = serde_yaml::from_str(&yaml);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "deserializing from YAML containing more than one document is not supported"
                .to_string()
        );
    }
}
