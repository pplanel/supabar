use thiserror::Error;
mod application {}
mod files {}

//pub trait Parser {
//    fn parse(input: &str) -> Result<Action, ModeParseError>;
//}
//
//#[derive(Debug, Error)]
//pub enum ModeParseError {
//    #[error("Mode not found")]
//    NotFound(String),
//    #[error("Incomplete Mode input")]
//    Incomplete,
//}
//
//#[derive(Debug)]
//pub enum Modes {
//    Application(String),
//    File(String),
//}
//
//impl Parser for Modes {
//    fn parse(input: &str) -> Result<Action, ModeParseError> {
//        match Modes::from_str(input) {
//            Ok(cmd) => match cmd {
//                Modes::Application(app) => Ok(Action::OpenApp(app)),
//                Modes::File(args) => Ok(Action::OpenFile(args)),
//            },
//            Err(err) => Err(err),
//        }
//    }
//}
//
//impl FromStr for Modes {
//    type Err = ModeParseError;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        let mut parts: Vec<&str> = s.split(' ').collect();
//        let prefix = parts.remove(0);
//
//        if parts.is_empty() {
//            Ok(Self::Application(s.into()))
//        } else {
//            let arguments = parts.join(" ");
//            match prefix {
//                "open" | "find" => Ok(Self::File(arguments)),
//                _ => Err(Self::Err::NotFound(prefix.into())),
//            }
//        }
//    }
//}
