extern crate core as supabar;
mod repl;
use anyhow::Result;
use supabar::{Runtime, LocalInfo, UserSettings};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = LocalInfo::get();
    let user_settings = UserSettings::new(&config)?;

    let runtime = Runtime::builder()
        .with_config(config)
        .with_user_settings(user_settings);

    let (mut core, mut _core_events) = runtime.build().await?;
    core.setup().await?;
    let core_handler = core.get_handler();

    tokio::spawn(async move {
        core.start().await;
    });

    repl::run_cli(&core_handler).await;
    Ok(())
}
