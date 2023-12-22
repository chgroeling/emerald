/// This module contains tests for serializing and deserializing data using `serde_yaml`.
/// It demonstrates the conversion of various Rust data structures to and from YAML format.
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
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

    /// Tests deserialization of a YAML string into a custom struct `Frontmatter`.
    /// This test checks if a YAML string can be correctly deserialized into a struct,
    /// handling fields such as tags, aliases, creation and modification dates, and keywords.
    #[test]
    fn deserialize_yaml_frontmatter_to_dedicated_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Frontmatter {
            tags: String,
            aliases: Vec<String>,
            created: String,
            modified: String,
            keywords: String,
        }

        // You have some type.
        let yaml =
            "tags: \"#Typ/Notiz\"\naliases:\n- \"embedded\"\ncreated: \nmodified:\nkeywords:\n";

        // Deserialize it back to a Rust type.
        let frontmatter: Frontmatter = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(frontmatter.tags, "#Typ/Notiz");
        assert_eq!(frontmatter.aliases, ["embedded"]);
        assert_eq!(frontmatter.created, "");
        assert_eq!(frontmatter.modified, "");
        assert_eq!(frontmatter.keywords, "");
    }

    /// Tests deserialization of a YAML string with superfluous fields into a custom struct `Frontmatter`.
    /// This test ensures that extra fields in the YAML string that are not defined in the `Frontmatter` struct
    /// do not affect the deserialization process.
    #[test]
    fn deserialize_yaml_frontmatter_to_dedicated_struct_superflous_field_in_input() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Frontmatter {
            tags: String,
            aliases: Vec<String>,
            created: String,
            modified: String,
            keywords: String,
        }

        // You have some type.
        let yaml =
            "not_included1: blabla\ntags: \"#Typ/Notiz\"\naliases:\n- \"embedded\"\nnot_included2: blabla\ncreated: \nmodified:\nkeywords:\nnot_included3: blabla\n";

        // Deserialize it back to a Rust type.
        let frontmatter: Frontmatter = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(frontmatter.tags, "#Typ/Notiz");
        assert_eq!(frontmatter.aliases, ["embedded"]);
        assert_eq!(frontmatter.created, "");
        assert_eq!(frontmatter.modified, "");
        assert_eq!(frontmatter.keywords, "");
    }

    /// Tests error handling during deserialization of YAML frontmatter containing document separators.
    /// This test checks if the YAML deserializer correctly returns an error when trying to deserialize
    /// YAML frontmatter with document separators, which is not supported.
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
