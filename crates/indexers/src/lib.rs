use contracts::candidate::FileCandidate;
use csv_indexer::csv_indexer::CsvIndexer;
use exif_indexer::exif_indexer::ExifIndexer;
use mobile_net_v2_indexer::mobile_net_v2_indexer::MobileNetV2Indexer;
use pptx_indexer::pptx_indexer::PptxIndexer;
use spreadsheet_indexer::spreadsheet_indexer::SpreadsheetIndexer;
use std::collections::HashSet;
use std::ffi::OsString;
use std::iter::FromIterator;
use text_indexer::text_indexer::TextIndexer;

use contracts::indexer::{DocumentSchema, Indexer};

/// Container for all Indexers
pub struct Analyzer {
    indexers: Vec<Box<dyn Indexer>>,
    pub supported_extensions: HashSet<OsString>,
}

impl Analyzer {
    pub async fn analyze(&self, file_to_process: FileCandidate) -> Vec<DocumentSchema> {
        self.indexers
            .iter()
            .filter(|indexer| {
                indexer.supports_extension(&file_to_process.path.extension().unwrap())
            })
            .filter_map(|indexer| indexer.index_file(&file_to_process).ok())
            .collect()
    }
}

impl Default for Analyzer {
    #[cfg(not(target_os = "windows"))]
    fn default() -> Analyzer {
        use pdf_indexer::pdf_indexer::PdfIndexer;

        let indexers: Vec<Box<dyn Indexer>> = vec![
            Box::new(TextIndexer),
            Box::new(ExifIndexer),
            Box::new(PdfIndexer),
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
            supported_extensions,
            indexers,
        }
    }

    #[cfg(target_os = "windows")]
    fn default() -> Analyzer {
        let indexers: Vec<Box<dyn Indexer>> = vec![
            Box::new(TextIndexer),
            Box::new(ExifIndexer),
            Box::new(PdfIndexer),
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
            supported_extensions,
            indexers,
        }
    }
}
