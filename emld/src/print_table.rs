use std::collections::HashMap;

use chrono::prelude::*;
use emerald::{Note, Vault};

use crate::string_formatter::StringFormatter;

enum Property {
    Title,
    Modified,
    Created,
    Size,
    LinkCnt,
    BackLinkCnt,
}

impl Property {
    fn value(&self) -> &str {
        match self {
            Property::Title => "title",
            Property::Modified => "modified",
            Property::Created => "created",
            Property::Size => "size",
            Property::LinkCnt => "linkcnt",
            Property::BackLinkCnt => "backlinkcnt",
        }
    }
    fn from(inp: &str) -> Property {
        match inp {
            "title" => Property::Title,
            "modified" => Property::Modified,
            "created" => Property::Created,
            "size" => Property::Size,
            "linkcnt" => Property::LinkCnt,
            "backlinkcnt" => Property::BackLinkCnt,
            _ => panic!("undefined property"),
        }
    }
}

const AVAILABLE_PROPS: &[Property] = &[
    Property::Title,
    Property::Modified,
    Property::Created,
    Property::Size,
    Property::LinkCnt,
    Property::BackLinkCnt,
];

fn note_property_to_str(element: &Property, note: &Note, vault: &impl Vault) -> String {
    match element {
        Property::Title => note.title.clone(),
        Property::Modified => {
            let modified = Local
                .timestamp_opt(note.modified.get_raw_value(), 0)
                .unwrap();
            modified.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Property::Created => {
            let created = Local
                .timestamp_opt(note.created.get_raw_value(), 0)
                .unwrap();
            created.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Property::Size => note.size.to_string(),
        Property::LinkCnt => vault.get_links_of(note).count().to_string(),
        Property::BackLinkCnt => vault.get_backlinks_of(note).count().to_string(),
    }
}

pub fn print_table(vault: &impl Vault) {
    let expr_parser = StringFormatter::new();
    let format_string = "\
          %<(40, trunc)%(title)\
         |%<(19, trunc)%(modified)\
         |%<(19, trunc)%(created)\
         |%<(12, trunc)%(size)\
         |%<( 6, trunc)%(linkcnt)\
         |%<( 6, trunc)%(backlinkcnt)";

    // # print header
    let mut key_value_store = HashMap::<&str, String>::new();
    AVAILABLE_PROPS.iter().for_each(|element| {
        let out_str = element.value();
        key_value_store.insert(element.value(), out_str.to_string());
    });

    println!(
        "{}",
        expr_parser.replace_placeholders(&key_value_store, format_string)
    );

    // # Determine which placeholders in the given format string are valid
    let placeholders = expr_parser.extract_placeholder_keys(&key_value_store, format_string);
    let used_props: Vec<_> = placeholders
        .into_iter()
        .map(|placeholder| Property::from(&placeholder))
        .collect();

    let length_of_format = expr_parser.measure_lengths(&key_value_store, format_string);

    // # print separator - use valid placeholders for it
    used_props.iter().enumerate().for_each(|(idx, property)| {
        let bar = "=".repeat(length_of_format[idx + 1]);
        let out_str = bar;
        key_value_store.insert(property.value(), out_str);
    });

    println!(
        "{}",
        expr_parser.replace_placeholders(&key_value_store, format_string)
    );

    // # print content - use valid placeholders for it
    let mut key_value_store = HashMap::<&str, String>::new();
    for i in vault.flat_iter() {
        used_props.iter().for_each(|property| {
            let ref_cell = note_property_to_str(&property, &i, vault);
            let out_str = ref_cell;
            key_value_store.insert(property.value(), out_str);
        });
        println!(
            "{}",
            expr_parser.replace_placeholders(&key_value_store, format_string)
        );
    }
}
