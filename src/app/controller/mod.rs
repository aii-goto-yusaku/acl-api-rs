mod acl_controller;

use crate::app::controller::acl_controller::acl_controller_factory;
use crate::core::error::Error;
use actix_web::{get, web, web::ServiceConfig, HttpResponse, Responder};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(health);
    cfg.service(error);
    acl_controller_factory(cfg);
}

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
