use crate::{database::Database, Config};

pub struct Application {
    pub settings: Config,
    gui: String,
    pub database: Database,
}

impl Application {
    pub fn new() -> Self {
        Self {
            settings: todo!(),
            gui: todo!(),
            database: todo!(),
        }
    }

    pub fn run() -> Result<(), anyhow::Error> {
        // Launch Modes monitoring threads
        // Update caches
        // Listen to shortcut
        Ok(())
    }
}
