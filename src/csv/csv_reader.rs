use std::path::Path;
use crate::csv_reader::csv_error::CsvReadError;

pub struct ColumnData {
    pub header: String,
    pub values: Vec<String>,
}

pub fn read_column(
    path: &Path,
    column_name: &str,
) -> Result<ColumnData, CsvReadError> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();

    let column_index = headers
        .iter()
        .position(|header| header == column_name)
        .ok_or_else(|| CsvReadError::ColumnNotFound(column_name.to_owned()))?;

    let mut values = Vec::new();

    for result in reader.records() {
        let record = result?;

        if let Some(value) = record.get(column_index) {
            values.push(value.to_owned());
        }
    }

    Ok(ColumnData {
        header: column_name.to_owned(),
        values,
    })
}