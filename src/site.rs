use actix_web::HttpRequest;
use serde_derive::Serialize;

use actix_web::{web, Responder, web::Json, delete, get, post, put, HttpResponse};
use actix_web::middleware::Logger;
use sqlx::{PgPool, Postgres};
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

#[derive(Serialize)]
pub struct Device {
    name: String,
    domain: String,
    address: String,
    icon: String,
    comment: String
}
                                                                                                                        
pub async fn healthz() -> web::Json<HealthStatus> {
    web::Json(HealthStatus{status: HealthState::Up})
}

pub async fn root() -> web::Json<HealthStatus> {
    web::Json(HealthStatus{status: HealthState::Up})
}

pub async fn add_device(pool: web::Data<Postgres>, device: Json<Device>) {

}

#[get("/devices")]
pub async fn get_devices(pool: web::Data<PgPool>)   -> impl Responder {
    let mut devices =  Vec::new();
    devices.push(Device{
        name: String::from("Plex"),
        domain: String::from("plex.local"),
        address: String::from("10.0.0.103"),
        icon: String::from("NA"),
        comment: String::from("Media")
    });


    // let mut rows = sqlx::query("SELECT * FROM devices")
    //     .fetch_all(pool);

    // while let Some(row) = rows.try_next().await? {
    //     // map the row into a user-defined domain type
    //     let email: &str = row.try_get("email")?;
    // }

    HttpResponse::Ok().json(devices)


}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_devices);
}