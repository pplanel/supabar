#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use std::process::{Command, Stdio};
use std::{str::FromStr, sync::Arc};
use tracing::{debug, event, instrument, Level};

pub use config::Config;
use database::Database;
use gio::AppInfo;
use gio::{glib, prelude::AppInfoExt};
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use user_state::UserState;

mod config;
mod database;
pub mod logging;
mod modes;
mod user_state;

pub struct Handler {
    query_sender: UnboundedSender<ReturnableMessage<ClientQuery>>,
    command_sender: UnboundedSender<ReturnableMessage<ClientCommand>>,
}

impl Handler {
    pub async fn query(&self, query: ClientQuery) -> Result<Response, CoreError> {
        let (sender, recv) = oneshot::channel();
        self.query_sender
            .send(ReturnableMessage {
                data: query,
                return_sender: sender,
            })
            .unwrap_or(());
        recv.await.unwrap_or(Err(CoreError::QueryError))
    }

    pub async fn command(&self, command: ClientCommand) -> Result<Response, CoreError> {
        let (sender, recv) = oneshot::channel();
        self.command_sender
            .send(ReturnableMessage {
                data: command,
                return_sender: sender,
            })
            .unwrap_or(());

        recv.await.unwrap()
    }
}

#[derive(Debug)]
pub enum InternalEvent {
    JobIngest(String),
    JobComplete(String),
}

#[derive(Clone)]
pub struct Context {
    pub database: Arc<Database>,
    pub event_sender: mpsc::Sender<Event>,
    pub internal_sender: UnboundedSender<InternalEvent>,
}

impl Context {
    //pub fn spawn_job(&self, job: Box<dyn Job>) {
    //    self.internal_sender
    //        .send(InternalEvent::JobIngest(job))
    //        .unwrap_or_else(|e| {
    //            println!("Failed to spawn job. {:?}", e);
    //        });
    //}
    pub async fn emit(&self, event: Event) {
        self.event_sender.send(event).await.unwrap_or_else(|e| {
            println!("Failed to emit event. {:?}", e);
        });
    }
}
// a wrapper around external input with a returning sender channel for core to respond
#[derive(Debug)]
pub struct ReturnableMessage<D, R = Result<Response, CoreError>> {
    data: D,
    return_sender: oneshot::Sender<R>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Core {
    state: UserState,
    jobs: Vec<String>,
    query_channel: (
        UnboundedSender<ReturnableMessage<ClientQuery>>,
        UnboundedReceiver<ReturnableMessage<ClientQuery>>,
    ),
    command_channel: (
        UnboundedSender<ReturnableMessage<ClientCommand>>,
        UnboundedReceiver<ReturnableMessage<ClientCommand>>,
    ),
    event_sender: mpsc::Sender<Event>,
    internal_channel: (UnboundedSender<Event>, UnboundedReceiver<Event>),
}

impl Core {
    #[allow(clippy::unit_arg)]
    #[instrument]
    pub async fn new() -> (Core, mpsc::Receiver<Event>) {
        let (event_sender, event_recv) = mpsc::channel(100);

        let config = Config::new();
        let mut state = UserState::new(config).unwrap();
        let username = state.username.clone();
        state
            .load()
            .unwrap_or(println!("No state found, creating one."));
        state.save();

        let internal_channel = unbounded_channel::<Event>();

        let core = Core {
            state,
            jobs: Vec::default(),
            query_channel: unbounded_channel(),
            command_channel: unbounded_channel(),
            event_sender,
            internal_channel,
        };

        event!(Level::DEBUG, "event happend {}", username);

        (core, event_recv)
    }

    pub fn get_handler(&self) -> Handler {
        Handler {
            query_sender: self.query_channel.0.clone(),
            command_sender: self.command_channel.0.clone(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.query_channel.1.recv() => {
                    debug!("Query received");
                    let response = self.exec_query(msg.data).await;
                    msg.return_sender.send(response).unwrap_or(());
                }
                Some(msg) = self.command_channel.1.recv() => {
                    debug!("Command received");
                    let response = self.exec_command(msg.data).await;
                    msg.return_sender.send(response).unwrap_or(());

                }
            }
        }
    }
    #[instrument(skip(self))]
    pub async fn exec_query(&self, query: ClientQuery) -> Result<Response, CoreError> {
        Ok(match query {
            ClientQuery::ListApplications => Response::ListApplications {
                applications: AppInfo::all()
                    .iter()
                    .map(|app_info| app_info.name().to_string())
                    .collect(),
            },
            _ => panic!("asdas"),
        })
    }
    #[instrument(skip(self))]
    pub async fn exec_command(&self, command: ClientCommand) -> Result<Response, CoreError> {
        match command {
            ClientCommand::OpenFile { path } => Ok(Response::OpenFile(path)),
            ClientCommand::SearchFile { term } => Ok(Response::SearchFile(Vec::from([term]))),
            ClientCommand::OpenApp { app_name } => {
                if let Some(app) = AppInfo::all()
                    .iter()
                    .find(|info| info.name().to_string().eq(&app_name))
                {
                    let cmd = app.executable();
                    debug!("cmd was ---- {:?}", &cmd.as_os_str());
                    Command::new(cmd)
                        .stderr(Stdio::null())
                        .stdout(Stdio::inherit())
                        .spawn()
                        .expect("Could not run app");
                    return Ok(Response::Success(()));
                };
                Err(CoreError::CommandError)
            }
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Log { message: String },
    DatabaseDisconnected { reason: Option<String> },
}

#[derive(Debug, PartialEq)]
pub enum ClientCommand {
    OpenFile { path: String },
    SearchFile { term: String },
    OpenApp { app_name: String },
}

#[derive(Debug, Error)]
pub enum ClientCommandParseError {
    #[error("Command not found")]
    NotFound,
}

impl FromStr for ClientCommand {
    type Err = ClientCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prefix: Vec<&str> = s.split(' ').collect();
        let args = prefix.split_off(1).join(" ");
        match prefix[0] {
            "open" => Ok(ClientCommand::OpenFile { path: args }),
            "search" => Ok(ClientCommand::SearchFile { term: args }),
            _ => Ok(ClientCommand::OpenApp { app_name: s.into() }),
        }
    }
}

#[derive(Debug)]
pub enum ClientQuery {
    ListApplications,
    ListFiles,
    SearchFiles,
    JobGetHistory,
    JobGetRunning,
}
#[derive(Debug)]
pub enum Response {
    ListApplications { applications: Vec<String> },
    OpenFile(String),
    SearchFile(Vec<String>),
    Success(()),
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Query error")]
    QueryError,
    #[error("Command error")]
    CommandError,
}

impl From<glib::Error> for CoreError {
    fn from(_: glib::Error) -> Self {
        Self::CommandError
    }
}
