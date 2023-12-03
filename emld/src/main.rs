mod format_option_parser;
mod print_table;

use clap;
use clap::{Parser, Subcommand};
use emerald::Emerald;
use emerald::EmeraldError;
use emerald::Result;
use emerald::Vault;

use format_option_parser::{FormatOptionParser, FormatOptions};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use print_table::print_table;
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
    List {
        #[arg(long, required=false, value_parser =FormatOptionParser, default_value="overview") ]
        format: FormatOptions,

        #[arg(long, required = false, default_value_t = false)]
        no_header: bool,
    },

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

fn uc_list(
    _vault_path: &Path,
    emerald: &Emerald,
    format_opt: &FormatOptions,
    print_header: bool,
) -> Result<()> {
    info!("Execute usecase: List");
    let format_string = match format_opt {
        FormatOptions::Overview => {
            "\
             %<(40, trunc)%(title)\
            |%<(19, trunc)%(modified)\
            |%<(19, trunc)%(created)\
            |%>(12, ltrunc)%(size)\
            |%>( 6, ltrunc)%(linkcnt)\
            |%>( 6, ltrunc)%(backlinkcnt)"
        }
        FormatOptions::Custom(custom_fmt_str) => custom_fmt_str,
    };

    let vault = emerald.get_vault();
    print_table(&vault, format_string, print_header);
    Ok(())
}

fn uc_all(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: All");
    let vault = emerald.get_vault();
    for i in vault.flat_iter() {
        println!("{} - {}", i.title, i.markdown);
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
        Commands::List { format, no_header } => uc_list(&vault_path, &emerald, format, !no_header)?,
    }
    debug!("User set vault path to {:?}", vault_path);

    let duration = start.elapsed();
    info!("Program execution took: {:?}", duration);
    Ok(())
}
