pub mod parser;

use std::path::{Path, PathBuf};

use anomali_rs::csv;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "anomali",
    version,
    about = "CLI to scan files for anomalies using benford's law"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Scan {
        file: PathBuf,

        #[arg(long)]
        column: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { file, column } => {
            csv_read_column(&file, &column);
        }
    }
}

fn csv_read_column(file: &Path, column: &str) {
    match csv::read_column(file, column) {
        Ok(data) => {
            println!(
                "Read {} rows from column {:?}",
                data.values.len(),
                data.header
            );
        }
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
