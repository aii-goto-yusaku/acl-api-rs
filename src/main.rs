mod app;
mod config;
mod core;

use crate::app::controller::app_config;
use actix_web::{App, HttpServer};
use anyhow::{Context, Result};
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

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

    HttpServer::new(|| App::new().wrap(TracingLogger).configure(app_config))
        .bind(format!("{}:{}", config.api_host, config.api_port))?
        .run()
        .await?;

    Ok(())
}
