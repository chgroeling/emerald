mod print_table;

use clap::{Parser, Subcommand};
use emerald::Emerald;
use emerald::EmeraldError;
use emerald::Result;
use emerald::Vault;

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

fn uc_list(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: List");
    let format_string = "\
         %<(40, trunc)%(title)\
        |%<(19, trunc)%(modified)\
        |%<(19, trunc)%(created)\
        |%>(12, ltrunc)%(size)\
        |%>( 6, ltrunc)%(linkcnt)\
        |%>( 6, ltrunc)%(backlinkcnt)";

    let vault = emerald.get_vault();
    print_table(&vault, format_string);
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
        Commands::List {} => uc_list(&vault_path, &emerald)?,
    }
    debug!("User set vault path to {:?}", vault_path);

    let duration = start.elapsed();
    info!("Program execution took: {:?}", duration);
    Ok(())
}
