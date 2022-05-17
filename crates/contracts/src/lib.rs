use anyhow::Result;

use std::ffi::{OsStr, OsString};

use blake2b_simd::blake2b;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::*;
use tracing::{info_span, instrument};
pub mod error {
    use tracing::error;

    pub fn log_and_return_error_string(error_string: String) -> String {
        error!("{}", error_string);
        error_string
    }
}
pub mod candidate {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct FileCandidate {
        pub path: std::path::PathBuf,
        pub hash: blake2b_simd::Hash,
        pub contents: Vec<u8>,
    }

    impl FileCandidate {
        pub fn path(&self) -> String {
            self.path.to_string_lossy().to_string()
        }
    }

    #[instrument]
    pub async fn new_file_to_process<T: AsRef<Path> + Debug>(path: T) -> FileCandidate
    where
        PathBuf: From<T>,
    {
        let contents = fs::read(&path).await.unwrap();

        let span = info_span!("calculating hash");
        let _enter = span.enter();
        let hash = calculate_hash(&contents);
        drop(_enter);

        FileCandidate {
            path: PathBuf::from(path),
            hash: hash,
            contents: contents,
        }
    }

    fn calculate_hash(input: &[u8]) -> blake2b_simd::Hash {
        let file_hash = blake2b(input);
        info!("Hash of file is: {:?}", file_hash);
        file_hash
    }
}
pub mod indexer {
    use super::*;
    use candidate::*;

    /// The schema of the information that an Indexer extracts from a file
    #[derive(serde::Serialize, serde::Deserialize, Debug, pallet::DocumentLike)]
    #[pallet(tree_name = "documents")]
    pub struct DocumentSchema {
        #[pallet(default_search_field)]
        pub name: String,
        #[pallet(default_search_field)]
        pub body: String,
        #[pallet(default_search_field)]
        pub media_type: String,
        #[pallet(default_search_field)]
        pub path: String,
    }

    /// Each Indexer needs to be able to say if a file extension is supported and extract information from a supported file
    pub trait Indexer: Send + Sync {
        /// If the Indexer supports a file extension
        /// Eg: PdfIndexer supports .pdf extensions
        fn supports_extension(&self, extension: &OsStr) -> bool;

        /// The logic behind the Indexer to extract information from a file
        fn index_file(&self, file_to_process: &FileCandidate) -> Result<DocumentSchema>;

        fn supported_extensions(&self) -> Vec<OsString>;
    }
}
