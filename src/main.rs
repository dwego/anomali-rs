pub mod parser;

use std::path::PathBuf;

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
            println!("File: {}", file.display());
            println!("Column: {column}");
        }
    }
}