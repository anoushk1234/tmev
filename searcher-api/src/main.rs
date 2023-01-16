mod api;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::*;
use repository::BundleRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = BundleRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_bundle_stats)
            .service(get_all_bundle_stats)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
