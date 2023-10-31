use emerald::{Note, Vault};

struct TableRow {
    max_width: usize,
    element: &'static str,
}

// TODO: const array
// TODO: array init
const TABLE_DEF: &[TableRow] = &[
    TableRow {
        max_width: 15,
        element: "title",
    },
    TableRow {
        max_width: 5,
        element: "title",
    },
    TableRow {
        max_width: 25,
        element: "title",
    },
];

const TRAIL: &str = "...";

fn note_element_2_str(note: &Note, element: &str) -> String {
    match element {
        "title" => note.title(),
        _ => panic!("Unknown element"),
    }
}

fn truncate_string(inp: &str, max_len: usize, trailing_str: &str) -> String {
    if inp.len() > max_len {
        let max_len_with_trail = max_len - trailing_str.len();
        format!("{0}{1}", &inp[0..max_len_with_trail], trailing_str)
    } else {
        inp.to_string()
    }
}

fn print_cell(content: &str, max_width: usize, trailing_str: &str) {
    let trimed_content = truncate_string(content, max_width, trailing_str);
    print!("{0:<1$}|", trimed_content, max_width);
}

pub fn print_table(vault: &impl Vault) {
    // print header
    print!("|");
    TABLE_DEF
        .iter()
        .for_each(|cell_def| print_cell(cell_def.element, cell_def.max_width, TRAIL));
    println!();

    // print separator
    print!("|");
    TABLE_DEF.iter().for_each(|cell_def| {
        let bar = "=".repeat(cell_def.max_width);
        print_cell(&bar, cell_def.max_width, TRAIL);
    });
    println!();

    // print content
    for i in vault.flat_iter() {
        print!("|");
        TABLE_DEF.iter().for_each(|cell_def| {
            let ref_cell = note_element_2_str(&i, cell_def.element);
            print_cell(&ref_cell, cell_def.max_width, TRAIL)
        });
        println!();
    }
}
