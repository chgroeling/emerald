use clap::{Parser, Subcommand};
use emerald::Emerald;
use emerald::EmeraldError;
use emerald::Note;
use emerald::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to note vault
    #[arg(required = true)]
    vault_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Return various statistics
    Stats {},

    /// Lists all notes as table.
    List {},

    /// Shows all notes
    All {},
}

fn uc_stats(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: Stats");

    println!("Md file count: {:?}", emerald.md_file_count());

    println!("Valid backlink count: {:?}", emerald.valid_backlink_count());

    println!(
        "Invalid backlink count: {:?}",
        emerald.invalid_backlink_count()
    );

    Ok(())
}

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

fn uc_list(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: List");
    let vault = emerald.get_vault();

    let trail: &'static str = "...";
    // print header
    print!("|");
    TABLE_DEF
        .iter()
        .for_each(|cell_def| print_cell(cell_def.element, cell_def.max_width, trail));
    println!("");

    // print separator
    print!("|");
    TABLE_DEF.iter().for_each(|cell_def| {
        let bar = "=".repeat(cell_def.max_width);
        print_cell(&bar, cell_def.max_width, trail);
    });
    println!("");

    // print content
    for i in vault.flat_iter() {
        print!("|");
        TABLE_DEF.iter().for_each(|cell_def| {
            let ref_cell = note_element_2_str(&i, cell_def.element);
            print_cell(&ref_cell, cell_def.max_width, trail)
        });
        println!("");
    }
    Ok(())
}

fn uc_all(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: All");
    let vault = emerald.get_vault();
    for i in vault.flat_iter() {
        println!("{} - {}", i.title(), i.markdown());
    }
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    info!("emeraldrs: Program start");

    let start = Instant::now();

    let vault_path = cli.vault_path.unwrap();

    if !vault_path.is_dir() {
        return Err(EmeraldError::VaultNotFound);
    }

    let emerald = Emerald::new(&vault_path)?;

    // execute use-cases
    match &cli.command {
        Commands::Stats {} => uc_stats(&vault_path, &emerald)?,
        Commands::All {} => uc_all(&vault_path, &emerald)?,
        Commands::List {} => uc_list(&vault_path, &emerald)?,
    }
    debug!("User set vault path to {:?}", vault_path);

    let duration = start.elapsed();
    info!("Program execution took: {:?}", duration);
    Ok(())
}
