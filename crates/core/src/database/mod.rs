use anyhow::Result;
use once_cell::sync::OnceCell;
use sea_orm::{prelude::*, Database as SeaDatabase};
use std::{path::PathBuf, sync::Arc};

use crate::user_state;
#[derive(Debug, Default)]
pub struct Database {
    url: String,
}

lazy_static! {
    pub static ref DB: OnceCell<Arc<DatabaseConnection>> = OnceCell::new();
}

pub fn get() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized")
}

impl Database {
    pub fn new(is_test: bool) -> Self {
        let url = if is_test {
            "sqlite::memory:".into()
        } else {
            format!(
                "sqlite://{}?mode=rwc",
                user_state::STATE
                    .get()
                    .unwrap()
                    .data_dir()
                    .join("db.sqlite")
                    .to_str()
                    .unwrap()
            )
        };

        Self {
            url,
            ..Default::default()
        }
    }

    pub async fn connect(&self) -> Result<DatabaseConnection> {
        Ok(SeaDatabase::connect(&self.url).await?)
    }
}

pub struct Index;
impl Index {
    pub(crate) fn create_user_index(_config: &crate::LocalInfo) -> std::path::PathBuf {
        PathBuf::new()
    }
}
