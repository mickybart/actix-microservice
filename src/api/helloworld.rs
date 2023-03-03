use actix_web::{get, web};
use tracing::instrument;
use serde::Deserialize;

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
    times: Option<u8>
}

#[get("/slowworld")]
async fn slow_world(info: web::Query<SlowQuery>) -> &'static str {
    let times = info.times.unwrap_or(2);

    for _ in 0..times {
        slow_down().await;
    }

    "Hellooooo, Wooooorld"
}

#[instrument]
async fn slow_down() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
