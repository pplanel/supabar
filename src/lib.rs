#[macro_use]
extern crate lazy_static;

pub use config::Config;
use jobs::Jobs;
pub use modes::Modes;
use modes::Parser;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use user_state::UserState;

mod config;
mod database;
mod jobs;
mod modes;
mod user_state;

pub struct CoreHandler {
    query_sender: UnboundedSender<ReturnableMessage<ClientQuery>>,
    command_sender: UnboundedSender<ReturnableMessage<ClientCommand>>,
}
// a wrapper around external input with a returning sender channel for core to respond
#[derive(Debug)]
pub struct ReturnableMessage<D, R = Result<CoreResponse, CoreError>> {
    data: D,
    return_sender: oneshot::Sender<R>,
}

pub struct Core {
    state: UserState,
    jobs: Jobs,
    query_channel: (
        UnboundedSender<ReturnableMessage<ClientQuery>>,
        UnboundedReceiver<ReturnableMessage<ClientQuery>>,
    ),
    command_channel: (
        UnboundedSender<ReturnableMessage<ClientCommand>>,
        UnboundedReceiver<ReturnableMessage<ClientCommand>>,
    ),
    event_sender: mpsc::Sender<CoreEvent>,
}

pub fn execute(input: &str) -> Result<String, String> {
    match Modes::parse(input) {
        Ok(action) => match action {
            Action::OpenFile(args) => Ok(format!("Opening file {}", args)),
            Action::OpenApp(app) => Ok(format!("Opening app {}", app)),
        },
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub enum CoreEvent {
    Log { message: String },
    DatabaseDisconnected { reason: Option<String> },
}

pub enum ClientCommand {
    OpenFile { path: String },
    SearchFile { term: String },
    OpenApp { app_name: String },
}

pub enum ClientQuery {
    ListApplications,
    ListFiles,
    SearchFiles,
}
pub enum CoreResponse {
    ListApplications,
    FindFiles,
    OpenFile,
}
pub enum CoreError {}
