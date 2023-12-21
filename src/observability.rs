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
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_resource(opentelemetry_sdk::Resource::default())
                .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry_sdk::runtime::TokioCurrentThread)
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
    opentelemetry_sdk::metrics::MeterProvider,
) {
    use opentelemetry_sdk::metrics::{Aggregation, Instrument, MeterProvider, Stream};

    let (metrics_handler, meter_provider) = {
        let registry = prometheus::Registry::new();
        let exporter = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build()
            .expect("Metrics exporter failure !");

        let provider = MeterProvider::builder()
            .with_reader(exporter)
            .with_view(
                opentelemetry_sdk::metrics::new_view(
                    Instrument::new().name("http.server.duration"),
                    Stream::new().aggregation(Aggregation::ExplicitBucketHistogram {
                        boundaries: vec![
                            0.0, 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5,
                            5.0, 7.5, 10.0,
                        ],
                        record_min_max: true,
                    }),
                ).unwrap()
            )
            .build();

        opentelemetry::global::set_meter_provider(provider.clone());

        (actix_web_opentelemetry::PrometheusMetricsHandler::new(registry), provider)
    };

    (metrics_handler, meter_provider)
}
