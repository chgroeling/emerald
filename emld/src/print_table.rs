use chrono::prelude::*;
use emerald::{Note, Vault};
use formatify::{Formatify, PlaceholderFormatter};
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Property {
    Title,
    Modified,
    Created,
    Size,
    LinkCnt,
    BackLinkCnt,
    Location,
    Markdown,
    Undefined,
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
            Property::Location => "location",
            Property::Markdown => "markdown",
            Property::Undefined => panic!("undefined property"),
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
            "location" => Property::Location,
            "markdown" => Property::Markdown,
            _ => Property::Undefined,
        }
    }
}

fn note_property_to_str(element: &Property, note: &Note, vault: &impl Vault) -> String {
    match element {
        Property::Title => note.title.clone(),
        Property::Location => note.location.clone(),
        Property::Markdown => note.markdown.clone(),
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
        Property::Undefined => panic!("Undefined property"),
    }
}

pub fn print_table(
    vault: &impl Vault,
    format_string: &str,
    print_header: bool,
    title_regex_predicate: &Option<String>,
) {
    let expr_parser = Formatify::new();

    // # Determine which placeholders in the given format string are valid
    let placeholders = expr_parser.extract_placeholder_keys(format_string);
    let used_props: Vec<_> = placeholders
        .into_iter()
        .map(|placeholder| Property::from(&placeholder))
        .filter(|prop| prop != &Property::Undefined) // remove all undefined properties
        .collect();

    if print_header {
        let mut key_value_store = HashMap::<&str, String>::new();

        // # print header
        used_props.iter().for_each(|element| {
            let out_str = element.value();
            key_value_store.insert(element.value(), out_str.to_string());
        });

        println!(
            "{}",
            expr_parser.replace_placeholders(&key_value_store, format_string)
        );

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
    }

    let mut opt_regex: Option<Regex> = None;
    if let Some(title_regex_predicate) = title_regex_predicate {
        // Try to create a new Regex object and assign it to opt_regex
        opt_regex = Regex::new(title_regex_predicate).ok()
    }

    // # print content - use valid placeholders for it
    let mut key_value_store = HashMap::<&str, String>::new();
    for i in vault.flat_iter() {
        // Check if opt_regex is Some and if the regex matches the title of the current element
        if let Some(ref regex) = opt_regex {
            if !regex.is_match(&i.title) {
                continue;
            }
        }
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
