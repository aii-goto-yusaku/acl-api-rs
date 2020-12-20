mod config;

use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    r#"{"status":"ok"}"#
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::Config::from_env()
        .expect("Load configuration");
    println!("{:?}", config);

    HttpServer::new(|| App::new().service(health))
        .bind(format!("{}:{}", config.api_host, config.api_port))?
        .run()
        .await
}
