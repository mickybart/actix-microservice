use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use opentelemetry_otlp::WithExportConfig;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .service(health)
            .service(helloworld)
            .service(slow_world)
            .service(hello)
            .default_service(web::route().to(HttpResponse::MethodNotAllowed))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await?;

    stop_telemetry();

    Ok(())
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

#[get("/")]
async fn helloworld() -> &'static str {
    "Hello, World !"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> String {
    format!("Hello, {name} !")
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({
        "status": "ok"
    }))
}

#[get("/slowworld")]
async fn slow_world() -> &'static str {
    slow_down().await;
    slow_down().await;
    "Hellooooo, Wooooorld"
}

#[tracing::instrument]
async fn slow_down() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
