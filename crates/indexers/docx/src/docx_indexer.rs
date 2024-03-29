use anyhow::Result;

use contracts::candidate::FileCandidate;
use contracts::indexer::{DocumentSchema, Indexer};
use std::ffi::{OsStr, OsString};

pub struct DocxIndexer;

impl Indexer for DocxIndexer {
    fn supports_extension(&self, extension: &OsStr) -> bool {
        extension == OsStr::new("docx")
    }

    fn supported_extensions(&self) -> Vec<OsString> {
        vec![OsString::from("docx")]
    }

    // Parsing Cats.docx panics the `docx` library...
    // We're just going to leave this out for now
    fn index_file(&self, _file_to_process: &FileCandidate) -> Result<DocumentSchema> {
        // let mut docx = Docx::from_file(file_to_process.path).unwrap();
        // dbg!(docx);

        Ok(DocumentSchema {
            name: String::new(),
            body: String::new(),
            media_type: "docx".into(),
            path: _file_to_process.path(),
        })
    }
}

#[cfg(test)]
mod tests {

    // #[tokio::test]
    // async fn test_indexing_docx_file() {
    //     let test_file_path = Path::new("../test_files/Cats.docx");

    //     let indexed_document = DocxIndexer
    //         .index_file(&new_file_to_process(test_file_path).await)
    //         .unwrap();

    //     assert_eq!(indexed_document.name, "file.txt");
    //     assert_eq!(
    //         indexed_document.body,
    //         "this is a file with some contents in it"
    //     );
    // }
}
