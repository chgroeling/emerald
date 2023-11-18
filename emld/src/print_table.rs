use std::collections::HashMap;

use chrono::prelude::*;
use emerald::{Note, Vault};

use crate::expr_parser::ExpressionParser;

enum Element {
    Title,
    Modified,
    Created,
    Size,
    LinkCnt,
    BackLinkCnt,
}

impl Element {
    fn value(&self) -> &str {
        match self {
            Element::Title => "title",
            Element::Modified => "modified",
            Element::Created => "created",
            Element::Size => "size",
            Element::LinkCnt => "linkcnt",
            Element::BackLinkCnt => "backlinkcnt",
        }
    }
}

const ELEMENT_DEF: &[Element] = &[
    Element::Title,
    Element::Modified,
    Element::Created,
    Element::Size,
    Element::LinkCnt,
    Element::BackLinkCnt,
];

fn note_element_2_str(element: &Element, note: &Note, vault: &impl Vault) -> String {
    match element {
        Element::Title => note.title.clone(),
        Element::Modified => {
            let modified = Local
                .timestamp_opt(note.modified.get_raw_value(), 0)
                .unwrap();
            modified.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Element::Created => {
            let created = Local
                .timestamp_opt(note.created.get_raw_value(), 0)
                .unwrap();
            created.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Element::Size => note.size.to_string(),
        Element::LinkCnt => vault.get_links_of(note).count().to_string(),
        Element::BackLinkCnt => vault.get_backlinks_of(note).count().to_string(),
    }
}

pub fn print_table(vault: &impl Vault) {
    let expr_parser = ExpressionParser::new();
    let format_string = "\
          %<(40, trunc)%(title)\
         |%<(19, trunc)%(modified)\
         |%<(19, trunc)%(created)\
         |%<(12, trunc)%(size)\
         |%<( 6, trunc)%(linkcnt)\
         |%<( 6, trunc)%(backlinkcnt)";

    // print header
    let mut key_value_store = HashMap::<&str, String>::new();
    ELEMENT_DEF.iter().for_each(|element| {
        let out_str = element.value();
        key_value_store.insert(element.value(), out_str.to_string());
    });
    println!("{}", expr_parser.format(&key_value_store, format_string));
    let length_of_format = expr_parser.measure(&key_value_store, format_string);

    // print separator
    ELEMENT_DEF.iter().enumerate().for_each(|(idx, element)| {
        let bar = "=".repeat(length_of_format[idx + 1]);
        let out_str = bar;
        key_value_store.insert(element.value(), out_str);
    });
    println!("{}", expr_parser.format(&key_value_store, format_string));

    // print content
    let mut key_value_store = HashMap::<&str, String>::new();
    for i in vault.flat_iter() {
        ELEMENT_DEF.iter().for_each(|elemet| {
            let ref_cell = note_element_2_str(elemet, &i, vault);
            let out_str = ref_cell;
            key_value_store.insert(elemet.value(), out_str);
        });
        println!("{}", expr_parser.format(&key_value_store, format_string));
    }
}
