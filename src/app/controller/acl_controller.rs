use crate::app::model::{relation_tuple, relation_tuple::{CheckQueryParam, User}};
use crate::core::error::Error;
use actix_web::{get, web, web::Query, web::ServiceConfig, HttpResponse, Responder};
use anyhow::{Context, Result};

pub fn acl_controller_factory(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/acl").service(check).service(checks));
}

#[get("/check")]
async fn check(Query(param): Query<CheckQueryParam>) -> Result<HttpResponse, Error> {
    let tuple = relation_tuple::parser::parse(&param.tuple)?;
    match tuple.user {
        User::UserID(_) => Ok(HttpResponse::Ok().json(param)),
        _ => Err(Error::ForbiddenError(format!("{:?}", tuple.user))),
    }
}

#[get("/checks")]
async fn checks() -> impl Responder {
    "checks"
}
