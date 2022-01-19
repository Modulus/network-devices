
use actix_web::{web};
use actix_web::middleware::Logger;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
pub mod site;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    // let cli_options = Options::from_args();

    // env_logger::Builder::from_env(Env::default().default_filter_or(cli_options.level)).init();
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://net:net@localhost/net?sslmode=disable").await.expect("Failed to create database pool");


    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL")
    // .expect("DATABASE_URL must be set");
    // // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool");


    // println!("Serving on {}", cli_options.bind);
    println!("Serving on {}", "127.0.0.1:3000");
    actix_web::HttpServer::new(move || {
        actix_web::App::new().data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .service(web::resource("/{id}/{name}/index.html").route(web::get().to(site::index)))
            .service(web::resource("/healthz").route(web::get().to(site::healthz)))
            .service(web::resource("/").route(web::get().to(site::root)))
            .configure(site::init)
            // .service(web::resource("/device").route(web::get().to(site::get_devices)))
    })
    // .bind(cli_options.bind)?
    .bind("127.0.0.1:3000")?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use crate::site::add_device;

    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        // let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let req = test::TestRequest::set_json(data);
        let resp = add_device(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // #[actix_rt::test]
    // async fn test_index_not_ok() {
    //     let req = test::TestRequest::default().to_http_request();
    //     let resp = index(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
}