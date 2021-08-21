use serde_derive::Serialize;

use actix_web::{web, Responder};
use actix_web::middleware::Logger;
use structopt::StructOpt;
use env_logger::Env;


async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[derive(Serialize)]
pub struct HealthStatus {
    status: HealthState,
}
                                                                                                                        
#[derive(Serialize)]
enum HealthState {
    Up,
    // Down, // Not in use yet
}
                                                                                                                        
pub async fn healthz() -> web::Json<HealthStatus> {
    web::Json(HealthStatus{status: HealthState::Up})
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    // let cli_options = Options::from_args();

    // env_logger::Builder::from_env(Env::default().default_filter_or(cli_options.level)).init();
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();


    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL")
    // .expect("DATABASE_URL must be set");
    // // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool");


    // println!("Serving on {}", cli_options.bind);
    println!("Serving on {}", "127.0.0.1:8080");
    actix_web::HttpServer::new(|| {
        // actix_web::App::new().data(Pool::builder().max_size(1).build(ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")
        // .expect("DATABASE_URL must be set"))).expect("Failed to create pool"))
            actix_web::App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::resource("/{id}/{name}/index.html").route(web::get().to(index)))
            .service(web::resource("/healthz").route(web::get().to(healthz)))
     
    })
    // .bind(cli_options.bind)?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}