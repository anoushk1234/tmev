mod api;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::*;
use repository::{BundleRepo, BundledTransactionRepo};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("gm")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "1");
    env_logger::init();

    let db = BundleRepo::init().await;
    let db_data = Data::new(db);
    let new_db = BundledTransactionRepo::init().await;
    let new_db_data = Data::new(new_db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_bundle_stats)
            .service(get_all_bundle_stats)
            .app_data(new_db_data.clone())
            .service(create_bundled_transactions)
            .service(get_all_bundle_transactions)
            .service(get_bundled_transaction_by_searcher)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
