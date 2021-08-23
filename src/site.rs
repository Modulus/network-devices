use std::future::Ready;

use actix_web::{HttpRequest, dev};
use serde_derive::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
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

#[post("/device")]
pub async fn add_device(pool: web::Data<PgPool>, device: Json<Device>) -> impl Responder {
    let rec = sqlx::query!(
        r#"INSERT INTO devices ( name, domain, address, icon, comment )
        VALUES ( $1, $2, $3, $4, $5)
        RETURNING name"#,
                device.name,
                device.domain,
                device.address,
                device.icon,
                device.comment
            )
            .fetch_one(pool.get_ref())
            .await.expect("Failed to insert device in database!");

            HttpResponse::Ok().json(rec.name)
}

#[get("/device")]
pub async fn get_devices(pool: web::Data<PgPool>)   -> impl Responder {
    let mut devices =  Vec::new();
    // devices.push(Device{
    //     name: String::from("Plex"),
    //     domain: String::from("plex.local"),
    //     address: String::from("10.0.0.103"),
    //     icon: String::from("NA"),
    //     comment: String::from("Media")
    // });


    let all_devices = sqlx::query!("SELECT * FROM devices")
        .fetch_all(pool.get_ref()).await.expect("Failed to fetch devices!");

    //TODO: Fix unwraps here
    for device in all_devices {
        devices.push(
            Device { 
                name: device.name, 
                domain: device.domain.unwrap(), 
                address: device.address.unwrap(), 
                icon: device.icon.unwrap(), 
                comment: device.comment.unwrap() }
        )
    }

    HttpResponse::Ok().json(devices)


}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_devices);
}