use anyhow::Result;
use gio::AppInfo;
use gio::{glib, prelude::AppInfoExt};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use tracing::{debug, info, instrument};

use crate::job::jobs::{Job, Jobs};
use crate::local_info::LocalInfo;
use crate::prisma::PrismaClient;
use crate::user_settings::UserSettings;
use crate::{database, user};

#[derive(Default, Debug)]
pub struct RuntimeBuilder {
    pub user_settings: UserSettings,
    pub config: LocalInfo,
}

impl RuntimeBuilder {
    pub fn with_config(mut self, config: LocalInfo) -> RuntimeBuilder {
        self.config = config;
        self
    }

    pub fn with_user_settings(mut self, user_settings: UserSettings) -> RuntimeBuilder {
        self.user_settings = user_settings;
        self
    }

    #[instrument]
    pub async fn build(self) -> Result<(Runtime, mpsc::Receiver<Event>)> {
        let (event_sender, event_recv) = mpsc::channel(100);

        let db = Arc::new(
            database::create_connection(&self.user_settings.data_dir.to_str().unwrap()).await?,
        );

        let internal_channel = unbounded_channel::<InternalEvent>();

        let core = Runtime {
            user_settings: self.user_settings,
            config: self.config,
            database: db,
            jobs: Jobs::new(),
            query_channel: unbounded_channel(),
            command_channel: unbounded_channel(),
            event_sender,
            internal_channel,
        };

        Ok((core, event_recv))
    }
}

#[allow(dead_code)]
pub struct Runtime {
    user_settings: UserSettings,
    config: LocalInfo,
    database: Arc<PrismaClient>,
    jobs: Jobs,
    query_channel: (
        UnboundedSender<ReturnableMessage<ClientQuery>>,
        UnboundedReceiver<ReturnableMessage<ClientQuery>>,
    ),
    command_channel: (
        UnboundedSender<ReturnableMessage<ClientCommand>>,
        UnboundedReceiver<ReturnableMessage<ClientCommand>>,
    ),
    event_sender: mpsc::Sender<Event>,
    internal_channel: (
        UnboundedSender<InternalEvent>,
        UnboundedReceiver<InternalEvent>,
    ),
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::default()
    }

    pub fn get_context(&self) -> Context {
        Context {
            database: self.database.clone(),
            event_sender: self.event_sender.clone(),
            internal_sender: self.internal_channel.0.clone(),
        }
    }

    pub fn get_handler(&self) -> Handler {
        Handler {
            query_sender: self.query_channel.0.clone(),
            command_sender: self.command_channel.0.clone(),
        }
    }

    pub async fn start(&mut self) {
        let ctx = self.get_context();
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
                Some(event) = self.internal_channel.1.recv() => {
                    match event {
                        InternalEvent::JobIngest(job) => {
                            self.jobs.ingest(&ctx, job).await;
                        },
                        InternalEvent::JobQueue(job) => {
                            self.jobs.ingest_queue(&ctx, job);
                        },
                        InternalEvent::JobComplete(id) => {
                            self.jobs.complete(&ctx, id).await;
                        },
                    }
                }
            }
        }
    }

    // Setup will perform some checks and spin jobs
    pub async fn setup(&self) {
        let ctx = self.get_context();
        // Get user indexable content
        match user::load_or_create(&ctx).await {
            Ok(_) => info!("Supabar initiated"),
            Err(er) => panic!("{}", format!("Some shit happend {}", er)),
        }
        // Create index for user
        // Index content
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
    JobIngest(Box<dyn Job>),
    JobQueue(Box<dyn Job>),
    JobComplete(String),
}

#[derive(Clone)]
pub struct Context {
    pub database: Arc<PrismaClient>,
    pub event_sender: mpsc::Sender<Event>,
    pub internal_sender: UnboundedSender<InternalEvent>,
}

impl Context {
    pub fn spawn_job(&self, job: Box<dyn Job>) {
        self.internal_sender
            .send(InternalEvent::JobIngest(job))
            .unwrap_or_else(|e| {
                println!("Failed to spawn job. {:?}", e);
            });
    }
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
#[derive(Debug)]
pub enum Event {
    Log { message: String },
    DatabaseDisconnected { reason: Option<String> },
    InvalidateQuery(ClientQuery),
    InvalidateQueryDebounced(ClientQuery),
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
