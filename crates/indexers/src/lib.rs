use exif_indexer::exif_indexer::ExifIndexer;
use mobile_net_v2_indexer::mobile_net_v2_indexer::MobileNetV2Indexer;
//use pdf_indexer::PdfIndexer;
use text_indexer::text_indexer::TextIndexer;
// pub use self::docx_indexer::DocxIndexer;
use contracts::candidate::FileCandidate;
use csv_indexer::csv_indexer::CsvIndexer;
use pptx_indexer::pptx_indexer::PptxIndexer;
use spreadsheet_indexer::spreadsheet_indexer::SpreadsheetIndexer;
use std::collections::HashSet;
use std::ffi::OsString;
use std::iter::FromIterator;

use once_cell::sync::Lazy;
use tokio;
use tracing::instrument;

use contracts::indexer::{DocumentSchema, Indexer};

/// Container for all Indexers
pub struct Analyzer {
    pub supported_extensions: HashSet<OsString>,
}

impl Default for Analyzer {
    #[cfg(not(target_os = "windows"))]
    fn default() -> Analyzer {
        let indexers: Vec<Box<dyn Indexer>> = vec![
            Box::new(TextIndexer),
            Box::new(ExifIndexer),
            // Box::new(PdfIndexer),
            Box::new(MobileNetV2Indexer),
            Box::new(PptxIndexer),
            Box::new(CsvIndexer),
            Box::new(SpreadsheetIndexer),
        ];

        let supported_extensions = HashSet::from_iter(
            indexers
                .iter()
                .map(|indexer| indexer.supported_extensions())
                .flatten(),
        );

        Analyzer {
            supported_extensions: supported_extensions,
        }
    }

    #[cfg(target_os = "windows")]
    fn default() -> Analyzer {
        let indexers: Vec<Box<dyn Indexer>> = vec![
            Box::new(TextIndexer),
            Box::new(ExifIndexer),
            // Box::new(PdfIndexer),
            Box::new(PptxIndexer),
            Box::new(CsvIndexer),
            Box::new(SpreadsheetIndexer),
        ];

        let supported_extensions = HashSet::from_iter(
            indexers
                .iter()
                .map(|indexer| indexer.supported_extensions())
                .flatten(),
        );

        Analyzer {
            supported_extensions: supported_extensions,
        }
    }
}

static INDEXERS: Lazy<Vec<Box<dyn Indexer>>> = Lazy::new(|| {
    let indexers: Vec<Box<dyn Indexer>> = vec![
        Box::new(TextIndexer),
        Box::new(ExifIndexer),
        // Box::new(PdfIndexer),
        Box::new(MobileNetV2Indexer),
        Box::new(PptxIndexer),
        Box::new(CsvIndexer),
        Box::new(SpreadsheetIndexer),
    ];
    indexers
});

#[instrument(skip(file_to_process))]
pub async fn analyze(extension: OsString, file_to_process: FileCandidate) -> Vec<DocumentSchema> {
    let processing_task = tokio::task::spawn_blocking(move || {
        INDEXERS
            .iter()
            .filter(|indexer| indexer.supports_extension(extension.as_os_str()))
            .filter_map(|indexer| indexer.index_file(&file_to_process).ok())
            .collect()
    });

    processing_task.await.unwrap()
}