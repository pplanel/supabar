use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::database::Index;
use crate::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserState {
    pub uuid: String,
    pub username: String,
    index: PathBuf,
    locations: HashMap<String, PathBuf>,
}

lazy_static! {
    static ref STATE: RwLock<Option<UserState>> = RwLock::new(None);
}

impl UserState {
    pub fn new(config: Config) -> Result<UserState> {
        let uuid = uuid::Uuid::new_v4();
        let username = whoami::username();
        let state = Self {
            uuid: uuid.to_string(),
            username,
            index: Index::create_user_index(&config),
            locations: config.to_hashmap(),
        };
        Ok(state)
    }

    pub fn save(&self) {
        self.write_memory();
        if let Some(app_data) = self.locations.get(&"AppDataLocation".to_string()) {
            let full_path = format!(
                "{}/client_{}_state.toml",
                app_data.to_str().unwrap(),
                self.uuid
            );
            let mut file = fs::File::create(full_path).unwrap();
            let contents = toml::to_string(&self).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        }
    }

    pub fn load(&mut self) -> Result<()> {
        let app_data = self.locations.get(&"AppDataLocation".to_string()).unwrap();

        let full_path = format!(
            "{}/client_{}_state.toml",
            app_data.to_str().unwrap(),
            self.uuid
        );
        let contents = std::fs::read_to_string(full_path)?;
        *self = toml::from_str(contents.as_str())?;
        Ok(())
    }

    fn write_memory(&self) {
        let mut writeable = STATE.write().unwrap();
        *writeable = Some(self.clone());
    }
}

#[test]
fn test_user_state() {
    let config = Config::new();
    if let Ok(mut state) = UserState::new(config) {
        state.save();
        state.load().unwrap();
    }
}
