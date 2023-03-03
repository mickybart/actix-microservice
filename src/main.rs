use actix_web::{get, HttpServer, App, web, Responder};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .service(health)
                .service(helloworld)
                .service(hello)
        }
    )
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
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
    web::Json(
        json!({
            "status": "ok"
        })
    )
}
