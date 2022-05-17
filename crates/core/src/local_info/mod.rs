use standard_paths::*;
use std::{collections::HashMap, path::PathBuf};

const APP_NAME: &str = "supabar";
const APP_ORG: &str = "superluminal";

pub struct LocalInfo {
    app_data: PathBuf,
    app_config: PathBuf,
}

impl LocalInfo {
    pub fn new() -> Self {
        let sl = StandardPaths::new(APP_NAME, APP_ORG);
        let app_data = sl
            .writable_location(LocationType::AppDataLocation)
            .expect("path dotnt exist");
        let config = sl
            .writable_location(LocationType::AppConfigLocation)
            .expect("df");
        for path in &[&app_data, &config] {
            if !&path.exists() {
                let _ = std::fs::create_dir_all(path.as_path());
            }
        }
        LocalInfo {
            app_data,
            app_config: config,
        }
    }

    pub fn to_hashmap(self) -> HashMap<String, PathBuf> {
        HashMap::from([
            (String::from("AppDataLocation"), self.app_data),
            (String::from("AppConfigLocation"), self.app_config),
        ])
    }
}
impl Default for LocalInfo {
    fn default() -> Self {
        Self::new()
    }
}
