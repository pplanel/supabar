use std::{env, path::PathBuf};

use crate::{prisma::user, runtime::Context, user_settings};

use thiserror::Error;
pub enum Platform {
    Unknown = 0,
    Windows = 1,
    MacOS = 2,
    Linux = 3,
    IOS = 4,
    Android = 5,
}
pub struct User {
    pub id: i32,
    pub username: String,
    pub index_dir: PathBuf,
    pub data_dir: PathBuf,
    pub platform: Platform,
}

pub async fn load_or_create(ctx: &Context) -> anyhow::Result<(), UserError> {
    let user_settings = user_settings::get();
    let db = &ctx.database;

    let platform = match env::consts::OS {
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "linux" => Platform::Linux,
        _ => Platform::Unknown,
    };

    let _user = match db
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
                user::index_dir::set(user_settings.index_dir.to_str().unwrap().to_string()),
                user::data_dir::set(user_settings.data_dir.to_str().unwrap().to_string()),
                vec![user::platform::set(platform as i32)],
            )
            .exec()
            .await
            .expect("cannot"),
    };

    Ok(())
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
    #[error("Client not found error")]
    ClientNotFound,
}
