use standard_paths::*;
use std::path::PathBuf;

use anyhow::Result;

const APP_NAME: &'static str = "supabar";
const APP_ORG: &'static str = "superluminal";

pub struct Config {
    app_data: PathBuf,
    config: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let sl = StandardPaths::new(APP_NAME, APP_ORG);
        let app_data = sl.writable_location(LocationType::AppDataLocation);
        let config = sl.writable_location(LocationType::AppConfigLocation);
        Config {
            app_data: app_data.unwrap(),
            config: config.unwrap(),
        }
    }

    fn setup_app_folders(self) -> Result<(), anyhow::Error> {
        Ok(for path in vec![self.app_data, self.config] {
                if !&path.exists() {
                    let _ = std::fs::create_dir_all(path.as_path());
                }
        })
    }
}
