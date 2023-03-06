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

#[cfg(feature = "metrics")]
pub fn init_metrics() -> (
    actix_web_opentelemetry::PrometheusMetricsHandler,
    actix_web_opentelemetry::RequestMetrics,
) {
    use opentelemetry::sdk::{
        export::metrics::aggregation,
        metrics::{controllers, processors, selectors},
    };

    let metrics_handler = {
        let controller = controllers::basic(
            processors::factory(
                selectors::simple::histogram([1.0, 2.0, 5.0, 10.0, 20.0, 50.0]),
                aggregation::cumulative_temporality_selector(),
            )
            .with_memory(true),
        )
        .build();

        let exporter = opentelemetry_prometheus::exporter(controller).init();
        actix_web_opentelemetry::PrometheusMetricsHandler::new(exporter)
    };

    let meter = opentelemetry::global::meter("actix_web");
    let request_metrics = actix_web_opentelemetry::RequestMetricsBuilder::new().build(meter);

    (metrics_handler, request_metrics)
}
