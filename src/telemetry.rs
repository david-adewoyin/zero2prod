use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(name: String, env_filter: String,sink:Sink) -> impl Subscriber where Sink:for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(),sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
    tracing_log::LogTracer::init().expect("Failed to set logger");
}
