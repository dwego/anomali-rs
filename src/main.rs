use std::path::PathBuf;
use std::process::ExitCode;

use anomali_rs::benford::BenfordAnalyzer;
use anomali_rs::csv;
use anomali_rs::parser::first_significant_digit;
use anomali_rs::report::terminal::render_benford_report;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "anomali",
    version,
    about = "Detect unusual numerical patterns in datasets"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan {
        /// Path to the CSV file
        file: PathBuf,

        /// Name of the numerical column to analyze
        #[arg(long)]
        column: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,

        Err(error) => {
            eprintln!();
            eprintln!("  error: {error}");
            eprintln!();

            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Scan { file, column } => scan(file, column),
    }
}

fn scan(
    file: PathBuf,
    column: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let column_data = csv::read_column(&file, &column)?;

    let mut analyzer = BenfordAnalyzer::new();

    for value in &column_data.values {
        analyzer.push(first_significant_digit(value));
    }

    let report = analyzer.finish()?;

    render_benford_report(
        &report,
        &file,
        &column_data.header,
    );

    Ok(())
}