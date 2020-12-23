mod config;
mod core;

use crate::core::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let config = config::Config::from_env().expect("Load configuration");
    println!("{:?}", config);

    HttpServer::new(|| App::new().service(health).service(error))
        .bind(format!("{}:{}", config.api_host, config.api_port))?
        .run()
        .await?;

    Ok(())
}
