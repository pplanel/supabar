use standard_paths::*;
use std::{collections::HashMap, path::PathBuf};

const APP_NAME: &'static str = "supabar";
const APP_ORG: &'static str = "superluminal";

pub struct Config {
    app_data: PathBuf,
    config: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let sl = StandardPaths::new(APP_NAME, APP_ORG);
        let app_data = sl
            .writable_location(LocationType::AppDataLocation)
            .expect("path dotnt exist");
        let config = sl
            .writable_location(LocationType::AppConfigLocation)
            .expect("df");
        for path in vec![&app_data, &config] {
            if !&path.exists() {
                let _ = std::fs::create_dir_all(path.as_path());
            }
        }
        Config { app_data, config }
    }

    pub fn to_hashmap(self) -> HashMap<String, PathBuf> {
        HashMap::from([
            (String::from("AppDataLocation"), self.app_data.clone()),
            (String::from("AppConfigLocation"), self.config.clone()),
        ])
    }
}
