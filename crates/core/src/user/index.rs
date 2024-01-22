use std::sync::{Arc, Mutex};

use crate::job::{jobs::Job, worker::WorkerContext};
use contracts::candidate::new_file_to_process;
use indexers::Analyzer;
use jwalk::WalkDirGeneric;
use tracing::{event, span, Level, debug};

use super::User;

#[derive(Debug, Default)]
struct State {
    indexable: bool,
}

pub struct IndexHome {
    pub(crate) user: User,
}
lazy_static! {
    static ref ANALYZER: Arc<Mutex<Analyzer>> = Arc::new(Mutex::new(Analyzer::default()));
}

#[async_trait::async_trait]
impl Job for IndexHome {
    async fn run(&self, _ctx: WorkerContext) -> anyhow::Result<()> {
        let initial_processing_span = span!(Level::INFO, "IndexHome job called");
        let _initial_processing_entry = initial_processing_span.enter();
        let walk_home = WalkDirGeneric::<((), State)>::new(&self.user.home_dir.clone())
            .follow_links(false)
            .skip_hidden(true)
            .parallelism(jwalk::Parallelism::RayonNewPool(4));

        let files: Vec<_> = walk_home
            .process_read_dir(move |_depth, _path, _read_dir_state, siblings| {
                let analyzer = Arc::clone(&ANALYZER);
                // siblings
                //     .iter_mut()
                //     .flatten()
                //     .map(|e| e.path())
                //     .filter_map(|e| e.extension().is_some())
                for entry in siblings.iter_mut().flatten() {
                    let file = entry.path();
                    if file.extension().is_some()
                        && analyzer
                            .lock()
                            .unwrap()
                            .supported_extensions
                            .contains(file.extension().unwrap())
                    {
                        entry.client_state = State { indexable: true }
                    }
                }
            })
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.client_state.indexable)
            .map(|e| e.path())
            .collect();
        event!(Level::DEBUG, "Files collected, ready to process.");

        let _docs = tokio::task::spawn(async move {
            event!(Level::DEBUG, "Processing files");

            let analyzer = Analyzer::default();
            let mut docs = Vec::new();
            for f in &files {
                let candidate = new_file_to_process(f).await;
                debug!("Got one candidate {candidate:?}");
                docs.extend(analyzer.analyze(candidate).await);
            }
            event!(Level::DEBUG, "Processing done");
            docs
        })
        .await?;
        debug!("This are the docs {_docs:?}");

        // self.user.store.inner.create_multi(&docs)?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "local_index"
    }
}
