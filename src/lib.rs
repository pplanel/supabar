#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

pub use config::Config;
use database::Database;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use user_state::UserState;

mod config;
mod database;
mod modes;
mod user_state;

pub struct CoreHandler {
    query_sender: UnboundedSender<ReturnableMessage<ClientQuery>>,
    command_sender: UnboundedSender<ReturnableMessage<ClientCommand>>,
}

impl CoreHandler {
    pub async fn query(&self, query: ClientQuery) -> Result<CoreResponse, CoreError> {
        let (sender, recv) = oneshot::channel();
        self.query_sender
            .send(ReturnableMessage {
                data: query,
                return_sender: sender,
            })
            .unwrap_or(());
        recv.await.unwrap_or(Err(CoreError::QueryError))
    }

    pub async fn command(&self, command: ClientCommand) -> Result<CoreResponse, CoreError> {
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
pub struct CoreContext {
    pub database: Arc<Database>,
    pub event_sender: mpsc::Sender<CoreEvent>,
    pub internal_sender: UnboundedSender<InternalEvent>,
}

impl CoreContext {
    //pub fn spawn_job(&self, job: Box<dyn Job>) {
    //    self.internal_sender
    //        .send(InternalEvent::JobIngest(job))
    //        .unwrap_or_else(|e| {
    //            println!("Failed to spawn job. {:?}", e);
    //        });
    //}
    pub async fn emit(&self, event: CoreEvent) {
        self.event_sender.send(event).await.unwrap_or_else(|e| {
            println!("Failed to emit event. {:?}", e);
        });
    }
}
// a wrapper around external input with a returning sender channel for core to respond
#[derive(Debug)]
pub struct ReturnableMessage<D, R = Result<CoreResponse, CoreError>> {
    data: D,
    return_sender: oneshot::Sender<R>,
}

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
    event_sender: mpsc::Sender<CoreEvent>,
}

#[derive(Debug)]
pub enum CoreEvent {
    // most all events should be once of these two
    InvalidateQuery(ClientQuery),
    InvalidateQueryDebounced(ClientQuery),
    NewThumbnail { cas_id: String },
    Log { message: String },
    DatabaseDisconnected { reason: Option<String> },
}

#[derive(Debug)]
pub enum ClientCommand {
    OpenFile { path: String },
    SearchFile { term: String },
    OpenApp { app_name: String },
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
pub enum CoreResponse {
    ListApplications,
    FindFiles,
    OpenFile,
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Query error")]
    QueryError,
}
