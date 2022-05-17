use tracing_subscriber::Layer;

pub struct LogLayer;

impl<S> Layer<S> for LogLayer where S: tracing::Subscriber {}
