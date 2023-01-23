use crate::{models::BundledTransaction, repository::BundledTransactionRepo};
use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde::Deserialize;
// FILE DEPRECATED!!!
#[post("/btxn/create")]
pub async fn create_bundled_transaction(
    db: Data<BundledTransactionRepo>,
    new_bundle: Json<BundledTransaction>,
) -> HttpResponse {
    let data = BundledTransaction {
        id: None,
        ..new_bundle.into_inner()
    };
    let bundle_detail = db.create_bundled_transaction_in_db(data).await;
    match bundle_detail {
        Ok(bundle) => HttpResponse::Ok().json(bundle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/btxn")]
pub async fn get_all_bundle_transactions(db: Data<BundledTransactionRepo>) -> HttpResponse {
    let bundle_stats = db.get_bundled_transactions_from_db().await;
    match bundle_stats {
        Ok(bundle) => HttpResponse::Ok().json(bundle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct QueryBtxnsBySearcher {
    searcher_key: String,
}

#[get("/btxn/searcher")]
async fn get_bundled_transaction_by_searcher(
    query: web::Query<QueryBtxnsBySearcher>,
    db: Data<BundledTransactionRepo>,
) -> HttpResponse {
    let searcher = query.searcher_key.clone();
    println!("searcher: {}", searcher);
    let btxn = db.get_bundled_transactions_by_searcher(&searcher).await;
    match btxn {
        Ok(btxn_bundle) => HttpResponse::Ok().json(btxn_bundle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/btxn/create_many")]
async fn create_bundled_transactions(
    db: Data<BundledTransactionRepo>,
    new_bundle_txns: Json<Vec<BundledTransaction>>,
) -> HttpResponse {
    let data = new_bundle_txns
        .iter()
        .map(|f| BundledTransaction {
            id: None,
            ..f.clone()
        })
        .collect::<Vec<BundledTransaction>>();
    let btxns = db.create_bundled_transactions_in_db(data).await;
    match btxns {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
