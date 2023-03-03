use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use env_logger::Env;
use opentelemetry_otlp::WithExportConfig;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .service(health)
            .service(helloworld)
            .service(hello)
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

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(opentelemetry::sdk::Resource::default())
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
        .expect("telemetry setup failure !");
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
