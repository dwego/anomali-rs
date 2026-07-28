use std::io::Write;
use tempfile::NamedTempFile;

fn temporary_csv(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("temporary file should be created");

    file.write_all(content.as_bytes())
        .expect("CSV content should be written");

    file.flush().expect("temporary CSV should be flushed");

    file
}

#[cfg(test)]
mod tests {
    use crate::temporary_csv;
    use anomali_rs::csv::{CsvReadError, read_column};

    #[test]
    fn preserves_missing_values() {
        let file = temporary_csv(
            "supplier,amount,date\n\
         Company A,1250.50,2026-07-01\n\
         Company B,,2026-07-02\n\
         Company C,4300.75,2026-07-03\n",
        );

        let result = read_column(file.path(), "amount").expect("amount column should be read");

        assert_eq!(result.values, vec!["1250.50", "", "4300.75"]);
    }

    #[test]
    fn handles_quoted_fields() {
        let file = temporary_csv(
            "supplier,amount\n\
         \"Company, Inc.\",1250.50\n\
         \"Example \"\"Quoted\"\" Company\",980.00\n",
        );

        let result = read_column(file.path(), "supplier").expect("supplier column should be read");

        assert_eq!(
            result.values,
            vec!["Company, Inc.", "Example \"Quoted\" Company",]
        );
    }

    #[test]
    fn returns_column_not_found_error() {
        let file = temporary_csv(
            "supplier,amount\n\
         Company A,1250.50\n",
        );

        let result = read_column(file.path(), "unknown");

        assert!(matches!(
            result,
            Err(CsvReadError::ColumnNotFound(column))
                if column == "unknown"
        ));
    }

    #[test]
    fn accepts_a_header_only_csv() {
        let file = temporary_csv("supplier,amount,date\n");

        let result = read_column(file.path(), "amount").expect("header-only CSV should be valid");

        assert!(result.values.is_empty());
    }

    #[test]
    fn returns_error_for_inconsistent_records() {
        let file = temporary_csv(
            "supplier,amount,date\n\
         Company A,1250.50,2026-07-01\n\
         Company B,980.00\n",
        );

        let result = read_column(file.path(), "amount");

        assert!(result.is_err());
    }

    #[test]
    fn returns_error_when_file_does_not_exist() {
        let path = std::path::Path::new("this-file-should-not-exist.csv");

        let result = read_column(path, "amount");

        assert!(result.is_err());
    }
}
