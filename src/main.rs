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
        app!()
            .wrap(Logger::default().log_target("http_log").exclude("/health"))
            .wrap(RequestTracing::new())
    })
    .bind(addr)?
    .run()
    .await?;

    logging::stop(&config);

    Ok(())
}

// WORKAROUND with macro as returning App from a function is not trivial (see multiple issues discussion on this topic)
#[macro_export]
macro_rules! app {
    () => {{
        let app = App::new()
            .service(health)
            .default_service(web::route().to(HttpResponse::MethodNotAllowed));

        #[cfg(feature = "helloworld")]
        let app = app.configure(crate::api::helloworld::register);

        app
    }};
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({
        "status": "ok"
    }))
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_health() {
        let app = App::new().service(health);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let bytes = to_bytes(resp.into_body()).await.unwrap();
        let json_body: serde_json::Value = serde_json::from_slice(&bytes[..]).unwrap();
        assert_eq!(
            json_body,
            json!(
                {
                    "status": "ok"
                }
            )
        );
    }

    #[actix_web::test]
    async fn test_default() {
        let app = test::init_service(app!()).await;

        let req = test::TestRequest::get().uri("/something/else").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_microservice() {
        let app = test::init_service(app!()).await;

        // default route
        let req = test::TestRequest::get().uri("/something/else").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());

        // GET /health
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let bytes = to_bytes(resp.into_body()).await.unwrap();
        let json_body: serde_json::Value = serde_json::from_slice(&bytes[..]).unwrap();
        assert_eq!(
            json_body,
            json!(
                {
                    "status": "ok"
                }
            )
        );

        #[cfg(feature = "helloworld")]
        {
            // GET /slowworld
            let req = test::TestRequest::get()
                .uri("/slowworld?times=0")
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            let bytes = to_bytes(resp.into_body()).await.unwrap();
            assert_eq!(bytes, b"Hellooooo, Wooooorld !"[..]);

            // GET /TEST
            let req = test::TestRequest::get().uri("/TEST").to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            let bytes = to_bytes(resp.into_body()).await.unwrap();
            assert_eq!(bytes, b"Hello, TEST !"[..]);

            // GET /
            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            let bytes = to_bytes(resp.into_body()).await.unwrap();
            assert_eq!(bytes, b"Hello, World !"[..]);
        }
    }
}
