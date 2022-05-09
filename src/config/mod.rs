use standard_paths::*;
use std::path::PathBuf;

use anyhow::Result;

const APP_NAME: &'static str = "supabar";
const APP_ORG: &'static str = "superluminal";

pub trait ConfigManager {
    fn load_config() -> Config;
    //fn setup_app_folders() -> Result<(), SetupFailed>;
    fn setup_app_folders() -> Result<()>;
}

pub struct Config {
    pub app_data: PathBuf,
    pub config: PathBuf,
}

impl ConfigManager for Config {
    fn load_config() -> Config {
        let sl = StandardPaths::new(APP_NAME, APP_ORG);
        let app_data = sl.writable_location(LocationType::AppDataLocation);
        let config = sl.writable_location(LocationType::AppConfigLocation);
        Config {
            app_data: app_data.unwrap(),
            config: config.unwrap(),
        }
    }

    fn setup_app_folders() -> Result<(), anyhow::Error> {
        let sl = StandardPaths::new(APP_NAME, APP_ORG);
        let app_data = sl.writable_location(LocationType::AppDataLocation);
        let config = sl.writable_location(LocationType::AppConfigLocation);

        Ok(for path in vec![app_data, config] {
            if let Ok(path) = path {
                if !&path.exists() {
                    let _ = std::fs::create_dir_all(path.as_path());
                }
            }
        })
    }
}
