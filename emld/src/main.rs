mod format_option_parser;
mod note_table_printer;
use clap::{Parser, Subcommand};
use emerald::Vault;
use format_option_parser::{FormatOptionParser, FormatOptions};
use note_table_printer::NoteTablePrinter;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use emerald::Emerald;
use emerald::EmeraldError;
use emerald::ResourceId;
use emerald::Result;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::note_table_printer::NoteTablePrinterConfig;

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

    Update {},

    /// Lists all notes as a table.
    List {
        /// Sets the format of the output table.
        #[arg(long, required=false, value_parser =FormatOptionParser, default_value="overview") ]
        format: FormatOptions,

        ///  If set, the output table will not include a header.
        #[arg(long, required = false, default_value_t = false)]
        no_header: bool,

        /// Determines how many links to follow for each note.
        #[arg(short = 'f', long, required = false, default_value_t = 0)]
        follow_links: u32,

        /// A regular expression to filter the notes.
        #[arg(short = 'r', long, required = false)]
        regex: Option<String>,
    },
}

fn uc_stats(emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: Stats");

    println!("Md file count: {:?}", emerald.md_file_count());

    println!("Valid backlink count: {:?}", emerald.valid_backlink_count());

    println!(
        "Invalid backlink count: {:?}",
        emerald.invalid_backlink_count()
    );

    Ok(())
}
fn uc_update(emerald: &Emerald) -> Result<()> {
    let vault = emerald.get_vault();
    for note in vault.flat_iter() {
        let note_resource_id: ResourceId = emerald
            .get_resource_id(&note)
            .ok_or(EmeraldError::ValueError)?;
    }
    Ok(())
}

fn uc_list(
    emerald: &Emerald,
    format_opt: &FormatOptions,
    print_header: bool,
    follow_links: u32,
    title_regex_predicate: &Option<String>,
) -> Result<()> {
    info!("Execute usecase: List");
    let format_string = match format_opt {
        FormatOptions::Overview => {
            if follow_links > 0 {
                "\
                %<(5, trunc)%(uid)\
                |%<( 1, trunc)%(depth)\
                |%<(40, trunc)%(title)\
                |%<(19, trunc)%(modified)\
                |%<(19, trunc)%(created)\
                |%>( 6, ltrunc)%(size)\
                |%>( 6, ltrunc)%(linkcnt)\
                |%>( 6, ltrunc)%(backlinkcnt)"
            } else {
                "\
                %<(5, trunc)%(uid)\
                |%<(40, trunc)%(title)\
                |%<(19, trunc)%(modified)\
                |%<(19, trunc)%(created)\
                |%>( 6, ltrunc)%(size)\
                |%>( 6, ltrunc)%(linkcnt)\
                |%>( 6, ltrunc)%(backlinkcnt)\
                |%<(30, trunc)%(keywords)"
            }
        }
        FormatOptions::ShowMarkdown => "%(markdown)%n",
        FormatOptions::Custom(custom_fmt_str) => custom_fmt_str,
    };

    let vault = emerald.get_vault();
    let note_table_printer_config = NoteTablePrinterConfig {
        format_string: format_string.to_string(),
        print_header,
        follow_links,
        title_regex_predicate: title_regex_predicate.clone(),
    };
    let pt = NoteTablePrinter {
        vault: &vault,
        config: note_table_printer_config,
    };
    pt.print();

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
        Commands::Stats {} => uc_stats(&emerald)?,
        Commands::Update {} => uc_update(&emerald)?,
        Commands::List {
            format,
            no_header,
            regex,
            follow_links,
        } => uc_list(&emerald, format, !no_header, *follow_links, regex)?,
    }
    debug!("User set vault path to {:?}", vault_path);

    let duration = start.elapsed();
    info!("Program execution took: {:?}", duration);
    Ok(())
}
