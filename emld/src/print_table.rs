use chrono::prelude::*;
use emerald::{Note, NoteTypes, Vault};
use formatify::{Formatify, PlaceholderFormatter};
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Property {
    Depth,
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
            Property::Depth => "depth",
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
            "depth" => Property::Depth,
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

fn note_property_to_str(element: &Property, note: &Note, vault: &dyn Vault, depth: u32) -> String {
    match element {
        Property::Depth => depth.to_string(),
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

struct PrintFollowLinks<'a> {
    vault: &'a dyn Vault,
    used_props: &'a Vec<Property>,
    format_string: &'a str,
    follow_links: u32,
}

impl<'a> PrintFollowLinks<'a> {
    fn print(&self, parent_note: &Note, depth: u32) {
        let mut key_value_store = HashMap::<&str, String>::new();
        let expr_parser = Formatify::new();
        for note_types in self.vault.get_links_of(parent_note) {
            let NoteTypes::Note(child) = note_types else {
                continue;
            };
            self.used_props.iter().for_each(|property| {
                let ref_cell = note_property_to_str(property, &child, self.vault, depth);
                let out_str = ref_cell;
                key_value_store.insert(property.value(), out_str);
            });
            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, self.format_string)
            );
            if self.follow_links > depth {
                self.print(&child, depth + 1);
            }
        }
    }
}

/// Represents a utility for printing information about notes in a table format.
///
/// `PrintTable` is designed to work with a `Vault` to retrieve notes and their properties,
/// format them according to a specified format string, and then print the formatted output.
/// It supports regex-based filtering of titles and can follow links to print related notes
/// to a specified depth.
///
/// # Fields
/// - `vault`: A reference to an object implementing the `Vault` trait, used for accessing notes.
/// - `format_string`: A string defining the format for printing each note's properties.
///   This string can contain placeholders that will be replaced with actual property values.
/// - `print_header`: A boolean flag to determine whether to print the table header.
/// - `follow_links`: A u32 value indicating the depth to which linked notes should be followed
///   and printed.
/// - `title_regex_predicate`: An optional string containing a regex pattern used to filter notes
///   by their titles. Only notes with titles matching the pattern will be printed.
///
///
/// Note that this struct requires a lifetime parameter `'a`, as it holds references.
pub struct PrintTable<'a> {
    pub vault: &'a dyn Vault,
    pub format_string: &'a str,
    pub print_header: bool,
    pub follow_links: u32,
    pub title_regex_predicate: &'a Option<String>,
}

impl<'a> PrintTable<'a> {
    pub fn print_table(&self) {
        let expr_parser = Formatify::new();

        // # Determine which placeholders in the given format string are valid
        let placeholders = expr_parser.extract_placeholder_keys(self.format_string);
        let used_props: Vec<_> = placeholders
            .into_iter()
            .map(|placeholder| Property::from(&placeholder))
            .filter(|prop| prop != &Property::Undefined) // remove all undefined properties
            .collect();

        if self.print_header {
            let mut key_value_store = HashMap::<&str, String>::new();

            // # print header
            used_props.iter().for_each(|element| {
                let out_str = element.value();
                key_value_store.insert(element.value(), out_str.to_string());
            });

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, self.format_string)
            );

            let length_of_format =
                expr_parser.measure_lengths(&key_value_store, self.format_string);

            // # print separator - use valid placeholders for it
            used_props.iter().enumerate().for_each(|(idx, property)| {
                let bar = "=".repeat(length_of_format[idx + 1]);
                let out_str = bar;
                key_value_store.insert(property.value(), out_str);
            });

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, self.format_string)
            );
        }

        let mut opt_regex: Option<Regex> = None;
        if let Some(title_regex_predicate) = self.title_regex_predicate {
            // Try to create a new Regex object and assign it to opt_regex
            opt_regex = Regex::new(title_regex_predicate).ok()
        }

        // # print content - use valid placeholders for it
        let mut key_value_store = HashMap::<&str, String>::new();
        for i in self.vault.flat_iter() {
            // Check if opt_regex is Some and if the regex matches the title of the current element
            if let Some(ref regex) = opt_regex {
                if !regex.is_match(&i.title) {
                    continue;
                }
            }

            used_props.iter().for_each(|property| {
                let ref_cell = note_property_to_str(property, &i, self.vault, 0);
                let out_str = ref_cell;
                key_value_store.insert(property.value(), out_str);
            });

            let pfl = PrintFollowLinks {
                vault: self.vault,
                used_props: &used_props,
                format_string: self.format_string,
                follow_links: self.follow_links,
            };

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, self.format_string)
            );

            if self.follow_links > 0 {
                pfl.print(&i, 1);
            }
        }
    }
}
