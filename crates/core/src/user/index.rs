use crate::job::jobs::Job;
#[derive(Debug)]
pub struct LocalIndex;

#[async_trait::async_trait]
impl Job for LocalIndex {
    async fn run(&self, _ctx: crate::job::worker::WorkerContext) -> anyhow::Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "user_local_index"
    }
}
