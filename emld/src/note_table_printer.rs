use chrono::prelude::*;
use emerald::{Note, NoteTypes, Vault};
use formatify::{Formatify, PlaceholderFormatter};
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq)]
enum NoteProperty {
    Depth,
    Title,
    Yaml,
    Modified,
    Created,
    Size,
    LinkCnt,
    BackLinkCnt,
    Location,
    Markdown,
    Aliases,
    Keywords,
    Undefined,
}

impl NoteProperty {
    fn value(&self) -> &str {
        match self {
            NoteProperty::Depth => "depth",
            NoteProperty::Title => "title",
            NoteProperty::Aliases => "aliases",
            NoteProperty::Keywords => "keywords",
            NoteProperty::Yaml => "yaml",
            NoteProperty::Modified => "modified",
            NoteProperty::Created => "created",
            NoteProperty::Size => "size",
            NoteProperty::LinkCnt => "linkcnt",
            NoteProperty::BackLinkCnt => "backlinkcnt",
            NoteProperty::Location => "location",
            NoteProperty::Markdown => "markdown",
            NoteProperty::Undefined => panic!("undefined property"),
        }
    }
    fn from(inp: &str) -> NoteProperty {
        match inp {
            "depth" => NoteProperty::Depth,
            "title" => NoteProperty::Title,
            "aliases" => NoteProperty::Aliases,
            "keywords" => NoteProperty::Keywords,
            "yaml" => NoteProperty::Yaml,
            "modified" => NoteProperty::Modified,
            "created" => NoteProperty::Created,
            "size" => NoteProperty::Size,
            "linkcnt" => NoteProperty::LinkCnt,
            "backlinkcnt" => NoteProperty::BackLinkCnt,
            "location" => NoteProperty::Location,
            "markdown" => NoteProperty::Markdown,
            _ => NoteProperty::Undefined,
        }
    }
}

fn note_property_to_str(
    element: &NoteProperty,
    note: &Note,
    vault: &dyn Vault,
    depth: u32,
) -> String {
    match element {
        NoteProperty::Depth => depth.to_string(),
        NoteProperty::Title => note.title.clone(),
        NoteProperty::Aliases => format!("{:?}", note.doc_metadata.aliases),
        NoteProperty::Yaml => note.yaml.clone(),
        NoteProperty::Keywords => format!("{:?}", note.doc_metadata.keywords.clone()),
        NoteProperty::Location => note.fs_metadata.location.clone(),
        NoteProperty::Markdown => note.markdown.clone(),
        NoteProperty::Modified => {
            let modified = Local
                .timestamp_opt(note.fs_metadata.modified.get_raw_value(), 0)
                .unwrap();
            modified.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        NoteProperty::Created => {
            let created = Local
                .timestamp_opt(note.fs_metadata.created.get_raw_value(), 0)
                .unwrap();
            created.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        NoteProperty::Size => note.fs_metadata.size.to_string(),
        NoteProperty::LinkCnt => vault.get_links_of(note).count().to_string(),
        NoteProperty::BackLinkCnt => vault.get_backlinks_of(note).count().to_string(),
        NoteProperty::Undefined => panic!("Undefined property"),
    }
}

struct NoteLinkTraversal<'a> {
    vault: &'a dyn Vault,
    used_props: &'a Vec<NoteProperty>,
    format_string: &'a str,
    follow_links: u32,
}

impl<'a> NoteLinkTraversal<'a> {
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

/// Configuration for `NoteTablePrinter`.
///
/// This struct holds all configuration options for `NoteTablePrinter`.
/// It allows for a flexible and clear setup of `NoteTablePrinter` by specifying
/// all relevant settings before instantiation.
///
/// # Fields
/// - `format_string`: A string that defines the format for printing each note's properties.
///   This string can contain placeholders that will be replaced with actual property values.
/// - `print_header`: A boolean value to determine whether to print the table header.
/// - `follow_links`: A u32 value indicating the depth to which linked notes should be followed
///   and printed.
/// - `title_regex_predicate`: An optional string containing a regex pattern used to filter notes
///   by their titles. Only notes with titles matching the pattern will be printed.
pub struct NoteTablePrinterConfig {
    pub format_string: String,
    pub print_header: bool,
    pub follow_links: u32,
    pub title_regex_predicate: Option<String>,
}

/// `NoteTablePrinter` - A utility for printing information about notes in a table format.
///
/// This struct works with a `Vault` to retrieve notes and their properties,
/// format them according to a specified format string, and print the formatted output.
/// It supports regex-based filtering of titles and can traverse links to print related notes
/// to a specified depth.
///
/// # Fields
/// - `vault`: A reference to an object implementing the `Vault` trait, used for accessing notes.
/// - `config`: Configuration settings for printing, including the format string, header printing option, link-following depth, and title regex filter.

pub struct NoteTablePrinter<'a> {
    pub vault: &'a dyn Vault,
    pub config: NoteTablePrinterConfig,
}

impl<'a> NoteTablePrinter<'a> {
    /// Prints a formatted table of notes from the vault.
    ///
    /// This method iterates over the notes in the vault and prints a formatted table based on
    /// the `NoteTablePrinterConfig`. The table includes details like note title, creation and
    /// modification dates, size, and link counts, depending on the configuration. It also supports
    /// depth-first traversal of linked notes up to a specified depth.
    ///
    /// The format of the table is defined by the `format_string` in `NoteTablePrinterConfig`.
    /// Optional regex-based title filtering is supported to include only specific notes.
    /// If `print_header` is set to `true` in the configuration, a header row is printed.
    pub fn print(&self) {
        let expr_parser = Formatify::new();

        // # Determine which placeholders in the given format string are valid
        let placeholders = expr_parser.extract_placeholder_keys(&self.config.format_string);
        let used_props: Vec<_> = placeholders
            .into_iter()
            .map(|placeholder| NoteProperty::from(&placeholder))
            .filter(|prop| prop != &NoteProperty::Undefined) // remove all undefined properties
            .collect();

        if self.config.print_header {
            let mut key_value_store = HashMap::<&str, String>::new();

            // # print header
            used_props.iter().for_each(|element| {
                let out_str = element.value();
                key_value_store.insert(element.value(), out_str.to_string());
            });

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, &self.config.format_string)
            );

            let length_of_format =
                expr_parser.measure_lengths(&key_value_store, &self.config.format_string);

            // # print separator - use valid placeholders for it
            used_props.iter().enumerate().for_each(|(idx, property)| {
                let bar = "=".repeat(length_of_format[idx + 1]);
                let out_str = bar;
                key_value_store.insert(property.value(), out_str);
            });

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, &self.config.format_string)
            );
        }

        let mut opt_regex: Option<Regex> = None;
        if let Some(title_regex_predicate) = self.config.title_regex_predicate.clone() {
            // Try to create a new Regex object and assign it to opt_regex
            opt_regex = Regex::new(&title_regex_predicate).ok()
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

            let pfl = NoteLinkTraversal {
                vault: self.vault,
                used_props: &used_props,
                format_string: &self.config.format_string,
                follow_links: self.config.follow_links,
            };

            println!(
                "{}",
                expr_parser.replace_placeholders(&key_value_store, &self.config.format_string)
            );

            if self.config.follow_links > 0 {
                pfl.print(&i, 1);
            }
        }
    }
}
