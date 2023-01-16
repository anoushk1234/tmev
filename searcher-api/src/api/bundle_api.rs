use crate::{models::BundleStat, repository::BundleRepo};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/bundle_stats/create")]
pub async fn create_bundle_stats(
    db: Data<BundleRepo>,
    new_bundle: Json<BundleStat>,
) -> HttpResponse {
    let data = BundleStat {
        id: None,
        ..new_bundle.into_inner()
    };
    let bundle_detail = db.create_bundle_in_db(data).await;
    match bundle_detail {
        Ok(bundle) => HttpResponse::Ok().json(bundle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/bundle_stats")]
pub async fn get_all_bundle_stats(db: Data<BundleRepo>) -> HttpResponse {
    let bundle_stats = db.get_bundles_from_db().await;
    match bundle_stats {
        Ok(bundle) => HttpResponse::Ok().json(bundle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
