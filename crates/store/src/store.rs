use anyhow::Result;
use std::path::PathBuf;
use tracing::instrument;

pub struct Store {}

impl Store {
    #[instrument]
    pub fn setup_store<T>(location: PathBuf) -> Result<pallet::Store<T>>
    where
        T: pallet::DocumentLike,
    {
        let db = sled::open(location.as_path().join("db"))?;
        let store = pallet::Store::builder()
            .with_db(db)
            .with_index_dir(location.as_path())
            .finish()?;
        Ok(store)
    }
}

#[tokio::test]
async fn test_store() {}
