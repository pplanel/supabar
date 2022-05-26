use crate::{prisma::user, runtime::Context, user_settings};
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
    pub store: Store,
}

impl Into<User> for user::Data {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            home_dir: PathBuf::from(self.home_dir),
            index_dir: PathBuf::from(&self.index_dir),
            data_dir: PathBuf::from(&self.data_dir),
            platform: self.platform.into(),
            store: Store::setup_store(
                PathBuf::from(&self.index_dir),
                PathBuf::from(self.index_dir),
            )
            .unwrap(),
        }
    }
}

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
pub async fn spawn_index_job(ctx: &Context) -> anyhow::Result<(), UserError> {
    let mut user_settings = user_settings::get();
    Ok(())
}

pub async fn load_or_create(ctx: &Context) -> anyhow::Result<User, UserError> {
    let mut user_settings = user_settings::get();
    let db = &ctx.database;

    let platform = match env::consts::OS {
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "linux" => Platform::Linux,
        _ => Platform::Unknown,
    };

    let user = match db
        .user()
        .find_unique(user::username::equals(user_settings.username.clone()))
        .exec()
        .await
        .expect("cannot find")
    {
        Some(user) => user,
        None => db
            .user()
            .create(
                user::username::set(whoami::username().clone()),
                user::home_dir::set(user_settings.home_dir.to_str().unwrap().to_string()),
                user::index_dir::set(user_settings.index_dir.to_str().unwrap().to_string()),
                user::data_dir::set(user_settings.data_dir.to_str().unwrap().to_string()),
                vec![user::platform::set(platform as i32)],
            )
            .exec()
            .await
            .expect("cannot"),
    };

    Ok(user.into())
}
