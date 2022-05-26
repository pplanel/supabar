use anyhow::Result;
use contracts::indexer::DocumentSchema;
use indexers::Analyzer;
use std::path::PathBuf;
use tracing::instrument;

pub struct Store {
    pub analyzer: Analyzer,
    pub inner: pallet::Store<DocumentSchema>,
}
impl Store {
    #[instrument]
    pub fn setup_store(db_dir: PathBuf, index_dir: PathBuf) -> Result<Store> {
        let temp_dir = tempfile::TempDir::new_in(".")?;
        let db = sled::open(temp_dir.path().join("db"))?;
        // let db = sled::open(db_dir.as_path().join("db"))?;
        let store = pallet::Store::builder()
            .with_db(db)
            .with_index_dir(temp_dir.path())
            .finish()?;

        Ok(Store {
            analyzer: Analyzer::default(),
            inner: store,
        })
    }
}

#[tokio::test]
async fn test_store() {}
