mod api;
mod config;
mod logging;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use serde_json::json;
use tracing::info;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::build();

    logging::init(&config);

    let addr = "0.0.0.0:3000";
    info!("listening on {}", addr);

    HttpServer::new(|| {
        let app = App::new()
            .wrap(Logger::default().log_target("http_log").exclude("/health"))
            .wrap(RequestTracing::new())
            .service(health)
            .default_service(web::route().to(HttpResponse::MethodNotAllowed));

        #[cfg(feature = "helloworld")]
        let app = app.configure(crate::api::helloworld::register);

        app
    })
    .bind(addr)?
    .run()
    .await?;

    logging::stop(&config);

    Ok(())
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({
        "status": "ok"
    }))
}
