use std::{collections::HashMap, path::PathBuf};
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

const APP_NAME: &str = "supabar";
const APP_ORG: &str = "superluminal";

#[derive(Debug)]
pub struct LocalInfo {
    app_data: PathBuf,
    app_config: PathBuf,
}

impl LocalInfo {
    pub fn get() -> Self {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_string(),
            author: "pplanel".to_string(),
            app_name: "supabar".to_string(),
        }).unwrap();

        let app_config = strategy.config_dir();
        let app_data = strategy.data_dir();

        LocalInfo {
            app_data,
            app_config,
        }
    }
    pub fn get_user_data_dir(&self, username: &str) -> PathBuf {
        let data_dir = PathBuf::from(format!("{}/{}/", self.app_data.to_string_lossy(), username));
        if !data_dir.exists() {
            let _ = std::fs::create_dir_all(&data_dir);
        }
        data_dir
    }
    pub fn get_user_index_dir(&self, username: &str) -> PathBuf {
        let user_index_dir = PathBuf::from(format!(
            "{}/{}/{}/",
            self.app_data.to_string_lossy(),
            username,
            "idx"
        ));

        if !user_index_dir.exists() {
            let _ = std::fs::create_dir_all(&user_index_dir);
        }
        user_index_dir
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
        Self::get()
    }
}
