use std::collections::HashMap;

use chrono::prelude::*;
use emerald::{Note, Vault};

use crate::expr_parser::ExpressionParser;

struct TableRow {
    element: &'static str,
}

const TABLE_DEF: &[TableRow] = &[
    TableRow { element: "title" },
    TableRow {
        element: "modified",
    },
    TableRow { element: "created" },
    TableRow { element: "size" },
    TableRow { element: "linkcnt" },
    TableRow {
        element: "backlinkcnt",
    },
];

fn note_element_2_str(note: &Note, vault: &impl Vault, element: &str) -> String {
    match element {
        "title" => note.title.clone(),
        "modified" => {
            let modified = Local
                .timestamp_opt(note.modified.get_raw_value(), 0)
                .unwrap();
            modified.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        "created" => {
            let created = Local
                .timestamp_opt(note.created.get_raw_value(), 0)
                .unwrap();
            created.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        "size" => note.size.to_string(),
        "linkcnt" => vault.get_links_of(note).count().to_string(),
        "backlinkcnt" => vault.get_backlinks_of(note).count().to_string(),
        _ => panic!("Unknown element"),
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
    TABLE_DEF.iter().for_each(|cell_def| {
        let out_str = cell_def.element;
        key_value_store.insert(cell_def.element, out_str.to_string());
    });
    println!("{}", expr_parser.parse(&key_value_store, format_string));
    let length_of_format = expr_parser.measure(&key_value_store, format_string);

    // print separator
    TABLE_DEF.iter().enumerate().for_each(|(idx, cell_def)| {
        let bar = "=".repeat(length_of_format[idx + 1]);
        let out_str = bar;
        key_value_store.insert(cell_def.element, out_str);
    });
    println!("{}", expr_parser.parse(&key_value_store, format_string));

    // print content
    let mut key_value_store = HashMap::<&str, String>::new();
    for i in vault.flat_iter() {
        TABLE_DEF.iter().for_each(|cell_def| {
            let ref_cell = note_element_2_str(&i, vault, cell_def.element);
            let out_str = ref_cell;
            key_value_store.insert(cell_def.element, out_str);
        });
        println!("{}", expr_parser.parse(&key_value_store, format_string));
    }
}
