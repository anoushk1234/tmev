use crate::{models::BlockBundles, repository::BlockBundlesRepo};
use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde::Deserialize;

#[post("/bundles/create")]
pub async fn create_one(db: Data<BlockBundlesRepo>, data: Json<BlockBundles>) -> HttpResponse {
    let data = BlockBundles {
        id: None,
        ..data.into_inner()
    };
    let res = db.create_one(data).await;
    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/bundles")]
pub async fn get_all(db: Data<BlockBundlesRepo>) -> HttpResponse {
    let res = db.get_all().await;
    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[derive(Deserialize)]
// pub struct Qy {
//     searcher_key: String,
// }

// #[get("/bundles/searcher")]
// async fn get_bundled_transaction_by_searcher(
//     query: web::Query<QueryBtxnsBySearcher>,
//     db: Data<BlockBundlesRepo>,
// ) -> HttpResponse {
//     let searcher = query.searcher_key.clone();
//     println!("searcher: {}", searcher);
//     let btxn = db.get_bundled_transactions_by_searcher(&searcher).await;
//     match btxn {
//         Ok(btxn_bundle) => HttpResponse::Ok().json(btxn_bundle),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

#[post("/bundles/create_many")]
async fn create_many(db: Data<BlockBundlesRepo>, data: Json<Vec<BlockBundles>>) -> HttpResponse {
    let data = data
        .iter()
        .map(|f| BlockBundles {
            id: None,
            ..f.clone()
        })
        .collect::<Vec<BlockBundles>>();
    let res = db.create_many(data).await;
    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
