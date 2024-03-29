use anyhow::{Context, Error, Result};
use contracts::candidate::FileCandidate;
use contracts::indexer::{DocumentSchema, Indexer};
use std::ffi::{OsStr, OsString};
use std::io::Cursor;
use tracing::{span, Level};

#[derive(Debug)]
pub struct CsvIndexer;

impl Indexer for CsvIndexer {
    fn supports_extension(&self, extension: &OsStr) -> bool {
        extension == OsStr::new("csv")
    }

    fn supported_extensions(&self) -> Vec<OsString> {
        vec![OsString::from("csv")]
    }

    fn index_file(&self, file_to_process: &FileCandidate) -> Result<DocumentSchema> {
        let path = file_to_process.path();
        span!(Level::INFO, "csv_indexer: indexing csv file ", "{}", path).in_scope(|| {
            let mut reader = span!(Level::INFO, "csv_indexer: Loading csv from memory")
                .in_scope(|| csv::Reader::from_reader(Cursor::new(&file_to_process.contents)));

            let headers = span!(Level::INFO, "csv_indexer: Processing csv info").in_scope(
                || -> Result<String, Error> {
                    Ok(reader
                        .headers()
                        .with_context(|| {
                            contracts::error::log_and_return_error_string(format!(
                                "csv_indexer: Failed to get headers from csv at path: {:?}",
                                file_to_process.path
                            ))
                        })?
                        .iter()
                        .fold(String::new(), |mut acc, x| {
                            acc.push_str(&x);
                            acc.push_str(" ");
                            acc
                        }))
                },
            )?;

            Ok(DocumentSchema {
                name: file_to_process.path(),
                body: headers,
                media_type: "test".into(),
                path: path.into(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use contracts::candidate::new_file_to_process;

    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_indexing_csv_file() {
        let test_file_path = Path::new("../test_files/data.csv");
        let indexed_document = CsvIndexer
            .index_file(&new_file_to_process(test_file_path).await)
            .unwrap();

        assert_eq!(indexed_document.name, "../../../test_files/data.csv");
        assert_eq!(
            indexed_document.body,
            "first_name last_name street city state postal_code "
        );
    }

    #[test]
    fn test_supports_csv_extension() {
        assert_eq!(true, CsvIndexer.supports_extension(OsStr::new("csv")));
        assert_eq!(false, CsvIndexer.supports_extension(OsStr::new("xslx")));
    }
}
