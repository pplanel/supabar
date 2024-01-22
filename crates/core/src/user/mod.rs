use crate::{runtime::Context, user_settings};
use std::{env, path::PathBuf};
use store::store::Store;
use thiserror::Error;
pub mod index;

pub struct User {
    pub id: i32,
    pub username: String,
    pub home_dir: PathBuf,
    pub index_dir: PathBuf,
    pub data_dir: PathBuf,
    pub platform: Platform,
    pub store: Option<Store>,
}

// impl Into<User> for user::Data {
//     fn into(self) -> User {
//         User {
//             id: self.id,
//             username: self.username,
//             home_dir: PathBuf::from(self.home_dir),
//             index_dir: PathBuf::from(&self.index_dir),
//             data_dir: PathBuf::from(&self.data_dir),
//             platform: self.platform.into(),
//             store: Store::setup_store(
//                 PathBuf::from(&self.index_dir),
//                 PathBuf::from(self.index_dir),
//             )
//             .unwrap(),
//         }
//     }
// }

pub enum Platform {
    Unknown = 0,
    Windows = 1,
    MacOS = 2,
    Linux = 3,
    IOS = 4,
    Android = 5,
}

impl From<i32> for Platform {
    fn from(val: i32) -> Self {
        match val {
            1 => Self::Windows,
            2 => Self::MacOS,
            3 => Self::Linux,
            4 => Self::IOS,
            5 => Self::Android,
            _ => Self::Unknown,
        }
    }
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
    #[error("Client not found error")]
    ClientNotFound,
}
pub async fn spawn_index_job(_ctx: &Context) -> anyhow::Result<(), UserError> {
    let _user_settings = user_settings::get();
    Ok(())
}

pub async fn load_or_create(_ctx: &Context) -> anyhow::Result<User, UserError> {
    let user_settings = user_settings::get();

    let platform = match env::consts::OS {
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "linux" => Platform::Linux,
        _ => Platform::Unknown,
    };

    Ok(User{
        id: 0,
        username: user_settings.username,
        home_dir: user_settings.home_dir,
        index_dir: user_settings.index_dir,
        data_dir: user_settings.data_dir,
        platform,
        store:None
    })
}
