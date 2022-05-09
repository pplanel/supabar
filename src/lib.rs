pub use modes::Modes;
use modes::{Action, Parser};

mod modes;

pub fn execute(input: &str) -> Result<String, String> {
    match Modes::parse(input) {
        Ok(action) => match action {
            Action::OpenFile(args) => Ok(format!("Opening file {}", args)),
            Action::OpenApp(app) => Ok(format!("Opening app {}", app)),
        },
        Err(err) => Err(format!("{:?}", err)),
    }
}
