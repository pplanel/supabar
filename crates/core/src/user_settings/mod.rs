use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::RwLock};
use uuid::Uuid;
use whoami::hostname;

use crate::local_info::LocalInfo;

// global, thread-safe storage for client state
lazy_static! {
    static ref CONFIG: RwLock<Option<UserSettings>> = RwLock::new(None);
}

pub fn get() -> UserSettings {
    match CONFIG.read() {
        Ok(guard) => guard.clone().unwrap_or(UserSettings::default()),
        Err(_) => return UserSettings::default(),
    }
}

// TODO(pplanel): create builder to defer in Runtime setup to check license
//                  and current installation
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserSettings {
    pub id: Option<i32>,
    pub username: String,
    pub hostname: String,
    pub index_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl UserSettings {
    pub fn new(config: &LocalInfo) -> Result<UserSettings> {
        let username = whoami::username();
        let state = Self {
            username: username.to_owned(),
            hostname: hostname(),
            index_dir: config.get_user_index_dir(&username),
            data_dir: config.get_user_data_dir(&username),
            id: None,
        };
        let mut writeable = CONFIG.write().unwrap();
        *writeable = Some(state.clone());
        Ok(state)
    }
}
