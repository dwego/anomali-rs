use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsvReadError {
    #[error("failed to open CSV file: {0}")]
    Io(#[from] io::Error),

    #[error("invalid CSV data: {0}")]
    Csv(#[from] csv::Error),

    #[error("column `{0}` was not found")]
    ColumnNotFound(String),
}