use gio::AppInfo;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    file_type: String,
}

#[derive(Debug, PartialEq)]
pub struct UserSettings {
    username: String,
    applications: Vec<AppInfo>,
    files: Vec<File>,
    locations: Vec<PathBuf>,
}

impl UserSettings {
    pub fn builder() -> UserSettingsBuilder {
        UserSettingsBuilder::default()
    }
}

#[derive(Default)]
pub struct UserSettingsBuilder {
    username: String,
    applications: Vec<AppInfo>,
    files: Vec<File>,
    locations: Vec<PathBuf>,
}

impl UserSettingsBuilder {
    pub fn new() -> UserSettingsBuilder {
        let username = whoami::username();
        UserSettingsBuilder {
            username,
            ..Default::default()
        }
    }

    pub fn build(self) -> UserSettings {
        UserSettings {
            username: self.username,
            applications: self.applications,
            files: self.files,
            locations: self.locations,
        }
    }

    pub fn add_application(mut self, app: AppInfo) -> UserSettingsBuilder {
        self.applications.push(app);
        self
    }

    pub fn add_file(mut self, file: File) -> UserSettingsBuilder {
        self.files.push(file);
        self
    }

    pub fn add_location(mut self, location: PathBuf) -> UserSettingsBuilder {
        self.locations.push(location);
        self
    }
}

mod collector {}
