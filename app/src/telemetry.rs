use std::collections::HashMap;
use std::fmt::Display;

use opentelemetry::KeyValue;
use opentelemetry::trace::{TraceError, TracerProvider as _};
use opentelemetry_otlp::{SpanExporterBuilder, WithExportConfig};
use opentelemetry_sdk::{Resource, runtime};
use opentelemetry_sdk::trace::{Config, TracerProvider};
use secrecy::ExposeSecret;
use serde::Deserialize;
use tokio::task::JoinHandle;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};
use tracing_subscriber::fmt::MakeWriter;
use crate::configuration::telemetry::TelemetryConfig;

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    telemetry_config: &TelemetryConfig,
    provider: &TracerProvider
) -> impl Subscriber + Sync + Send
    where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.clone().into(), sink);

    let open_telemetry = tracing_opentelemetry::layer()
        .with_tracer(provider.tracer(telemetry_config.dataset_name.clone()));

    return Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(tracing_subscriber::fmt::Layer::default())
        .with(open_telemetry)
}


/// Register a subscriber as global default to process span data.
///
/// NOTE: It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // LogTracer is a compatibility layer for integrating the log crate with the tracing crate.
    LogTracer::init().expect("Failed to set logger.");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn init_tracer(trace_config: &TelemetryConfig) -> Result<TracerProvider, TraceError> {
    // todo when going to prod, this code must be updated for specific vendor that will
    // todo receiving the traces.

    let span_exporter = opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint(trace_config.otlp_endpoint.expose_secret().clone())
            .with_http_client(reqwest::Client::default())
            .with_timeout(std::time::Duration::from_secs(2));

    Ok(TracerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME.to_string(),
                trace_config.dataset_name.clone(),
            )])),
        )
        .with_batch_exporter(
            SpanExporterBuilder::Http(span_exporter)
                .build_span_exporter()?,
            runtime::Tokio,
        )
        .build()
    )
}

/// spawn_blocking_with_tracing moves the current tracing Span into the newly created thread
/// running the blocking operation. This will allow the new tracing to inherit all the properties
/// of the current tracing within this thread.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

pub trait TelemetryRecord {
    fn record_in_telemetry(&self, name_of_field: &str);
}

impl<T: Display> TelemetryRecord for T {
    fn record_in_telemetry(&self, name_of_field: &str) {
        tracing::Span::current().record(name_of_field, &tracing::field::display(self));
    }
}
