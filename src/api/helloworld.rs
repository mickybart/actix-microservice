use actix_web::{get, web};
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

#[get("/slowworld")]
async fn slow_world() -> &'static str {
    slow_down().await;
    slow_down().await;
    "Hellooooo, Wooooorld"
}

#[instrument]
async fn slow_down() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
