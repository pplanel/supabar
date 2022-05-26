use anyhow::{Context, Error, Result};
use contracts::candidate::FileCandidate;
use contracts::indexer::{DocumentSchema, Indexer};
use lopdf::Document;
use std::ffi::{OsStr, OsString};
use tracing::{span, Level};

use regex::Regex;

#[derive(Debug)]
pub struct PdfIndexer;

impl Indexer for PdfIndexer {
    fn supports_extension(&self, extension: &OsStr) -> bool {
        extension == OsStr::new("pdf")
    }

    fn supported_extensions(&self) -> Vec<OsString> {
        vec![OsString::from("pdf")]
    }

    fn index_file(&self, file_to_process: &FileCandidate) -> Result<DocumentSchema> {
        let path = file_to_process.path.to_str().unwrap();
        span!(Level::INFO, "pdf_indexer: indexing pdf file", path).in_scope(|| {
            let res = span!(Level::INFO, "pdf_indexer: Loading from disk and processing")
                .in_scope(|| {
                    let document = Document::load(&path)?;
                    let pages = document.get_pages().into_keys().collect::<Vec<u32>>();
                    document.extract_text(&pages).with_context(|| {
                        contracts::error::log_and_return_error_string(format!(
                            "pdf_indexer: Failed to create regex"
                        ))
                    })
                })?;

            let clean = span!(Level::INFO, "pdf_indexer: Processing file").in_scope(
                || -> Result<String, Error> {
                    // THIS IS A BAD HACK
                    let re = Regex::new(r"\b ").with_context(|| {
                        contracts::error::log_and_return_error_string(format!(
                            "pdf_indexer: Failed to create regex"
                        ))
                    })?;

                    Ok(re.replace_all(&res, "").to_string())
                },
            )?;

            Ok(DocumentSchema {
                name: String::new(),
                body: clean,
                media_type: "pdf".into(),
                path: file_to_process.path(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_indexing_pdf_file() {
    //     let test_file_path = Path::new("./test_files/Cats.pdf");
    //     let indexed_document = PdfIndexer
    //         .index_file(&new_file_to_process(test_file_path).await)
    //         .unwrap();

    //     assert_eq!(indexed_document.name, "");
    //     assert_eq!(indexed_document.body, "\n\nCats \n\nThis  is  an  example  document about cats.  \n\n \n\nCats  have  paws.  ");
    // }

    #[test]
    fn test_supports_pdf_extension() {
        assert_eq!(true, PdfIndexer.supports_extension(OsStr::new("pdf")));
        assert_eq!(false, PdfIndexer.supports_extension(OsStr::new("docx")))
    }
}
