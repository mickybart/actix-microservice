use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;

pub fn init(config: &Config) {
    match config.telemetry {
        true => init_telemetry(),
        false => init_tracing(),
    }
}

pub fn stop(config: &Config) {
    match config.telemetry {
        true => stop_telemetry(),
        false => (),
    }
}

fn init_telemetry() {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry::sdk::propagation::TraceContextPropagator::new(),
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(opentelemetry::sdk::Resource::default())
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
        .expect("telemetry setup failure !");

    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let logs_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(telemetry)
        .with(logs_layer)
        .init()
}

fn stop_telemetry() {
    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();
}

fn init_tracing() {
    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));

    let logs_layer = tracing_subscriber::fmt::layer();
    // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(logs_layer)
        .init();
}
