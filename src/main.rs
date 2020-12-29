mod config;
mod core;

use crate::core::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[get("/health")]
async fn health() -> impl Responder {
    r#"{"status":"ok"}"#
}

#[derive(Serialize, Deserialize)]
struct ErrorEndpointPathParam {
    status_code: u32,
}
#[get("/verify/error/{status_code}")]
async fn error(path_param: web::Path<ErrorEndpointPathParam>) -> Result<HttpResponse, Error> {
    match path_param.status_code {
        400 => Err(Error::BadRequestError("test error".to_string())),
        401 => Err(Error::UnauthorizedError("test error".to_string())),
        404 => Err(Error::NotFoundError("test error".to_string())),
        500 | _ => Err(Error::InternalServerError("test error".to_string())),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::Config::from_env().expect("Load configuration");
    println!("{:?}", config);

    LogTracer::init().expect("Unable to setup log tracer");
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .context("Setting tracing subscriber global default")?;

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger)
            .service(health)
            .service(error)
    })
    .bind(format!("{}:{}", config.api_host, config.api_port))?
    .run()
    .await?;

    Ok(())
}
