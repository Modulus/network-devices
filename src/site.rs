use serde_derive::Serialize;

use actix_web::{web, Responder};
use actix_web::middleware::Logger;
use structopt::StructOpt;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[derive(Serialize)]
pub struct HealthStatus {
    status: HealthState,
}
                                                                                                                        
#[derive(Serialize)]
pub enum HealthState {
    Up,
    // Down, // Not in use yet
}
                                                                                                                        
pub async fn healthz() -> web::Json<HealthStatus> {
    web::Json(HealthStatus{status: HealthState::Up})
}

pub async fn root() -> web::Json<HealthStatus> {
    web::Json(HealthStatus{status: HealthState::Up})
}