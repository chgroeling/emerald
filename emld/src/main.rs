use clap::{Parser, Subcommand};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use emerald::Emerald;
use emerald::EmeraldError;
use emerald::Result;

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

fn uc_all(_vault_path: &Path, emerald: &Emerald) -> Result<()> {
    info!("Execute usecase: All");
    let vault = emerald.get_vault();
    for i in vault.flat_iter() {
        println!("{}", i.title());
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
    }
    debug!("User set vault path to {:?}", vault_path);

    let duration = start.elapsed();
    info!("Program execution took: {:?}", duration);
    Ok(())
}
