use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::database::Index;
use crate::Config;

struct File {
    name: String,
    path: PathBuf,
    file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserState {
    uuid: String,
    username: String,
    locations: HashMap<String, PathBuf>,
    index: PathBuf,
}

lazy_static! {
    static ref STATE: RwLock<Option<UserState>> = RwLock::new(None);
}

impl UserState {
    pub fn new() -> Result<UserState> {
        let uuid = uuid::Uuid::new_v4();
        let config = Config::new();
        let username = whoami::username();
        let state = Self {
            uuid: uuid.to_string(),
            username,
            index: Index::create_user_index(&config),
            locations: HashMap::new(),
        };
        Ok(state)
    }

    pub fn save(&self) {
        if self.locations.get()
    }
    pub fn load(&self) {}
}

#[test]
fn test_user_state() {
    let state = UserState::new();
}
