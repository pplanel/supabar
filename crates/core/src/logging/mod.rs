use tracing::Level;
use tracing_subscriber::{util::SubscriberInitExt, Layer};

pub struct LogLayer;

impl<S> Layer<S> for LogLayer where S: tracing::Subscriber {}

pub fn setup_global_subscriber() {
    let _t = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish()
        .try_init();
}
