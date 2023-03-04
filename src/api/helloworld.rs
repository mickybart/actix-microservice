use actix_web::{get, web};
use serde::Deserialize;
use tracing::instrument;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(helloworld).service(slow_world).service(hello);
}

#[get("/")]
async fn helloworld() -> &'static str {
    "Hello, World !"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> String {
    format!("Hello, {name} !")
}

#[derive(Deserialize)]
struct SlowQuery {
    times: Option<u8>,
}

#[get("/slowworld")]
async fn slow_world(info: web::Query<SlowQuery>) -> &'static str {
    let times = info.times.unwrap_or(2);

    for _ in 0..times {
        slow_down().await;
    }

    "Hellooooo, Wooooorld !"
}

#[instrument]
async fn slow_down() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_helloworld() {
        let app = App::new().service(helloworld);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(bytes, b"Hello, World !"[..]);
    }

    #[actix_web::test]
    async fn test_hello() {
        let app = App::new().service(hello);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/TEST").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(bytes, b"Hello, TEST !"[..]);
    }

    #[actix_web::test]
    async fn test_slowworld() {
        let app = App::new().service(slow_world);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get()
            .uri("/slowworld?times=0")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(bytes, b"Hellooooo, Wooooorld !"[..]);
    }

    #[actix_web::test]
    async fn test_register() {
        let app = App::new().configure(register);
        let app = test::init_service(app).await;

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
