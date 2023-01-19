extern crate dotenv;
use std::env;

use dotenv::dotenv;
use futures::{future::IntoStream, TryStreamExt};
use log::info;

use crate::models::BundledTransaction;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertManyResult, InsertOneResult},
    Client, Collection, Cursor,
};
// use env_logger::

pub struct BundledTransactionRepo {
    col: Collection<BundledTransaction>,
}

impl BundledTransactionRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("data");
        let col: Collection<BundledTransaction> = db.collection("BundledTransactions");
        // MongoRepo { col }
        BundledTransactionRepo { col }
    }
    pub async fn create_bundled_transaction_in_db(
        &self,
        new_bundle_txn: BundledTransaction,
    ) -> Result<InsertOneResult, Error> {
        let new_doc = BundledTransaction {
            id: None,
            ..new_bundle_txn
        };
        let bundled_txn = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating bundle in db");
        Ok(bundled_txn)
    }
    pub async fn create_bundled_transactions_in_db(
        &self,
        mut new_bundle_txns: Vec<BundledTransaction>,
    ) -> Result<InsertManyResult, Error> {
        let new_docs = new_bundle_txns
            .iter_mut()
            .map(move |f| BundledTransaction {
                id: None,
                ..f.clone()
            })
            .collect::<Vec<BundledTransaction>>();
        let bundled_txn = self
            .col
            .insert_many(new_docs, None)
            .await
            .ok()
            .expect("Error creating bundled txn in db");
        Ok(bundled_txn)
    }
    pub async fn get_bundled_transactions_from_db(&self) -> Result<Vec<BundledTransaction>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of btxns");
        let mut bundle_txns: Vec<BundledTransaction> = Vec::new();
        // println!("here {:?}", cursors.try_next().await);
        while let Some(txn) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            println!("here: {:?}", txn);
            bundle_txns.push(txn)
        }
        Ok(bundle_txns)
    }
    pub async fn get_bundled_transactions_by_searcher(
        &self,
        searcher_key: &String,
    ) -> Result<Vec<BundledTransaction>, Error> {
        println!("searcher_key1: {:?}", searcher_key);
        // let obj_id = ObjectId::parse_str("kash.sol").unwrap();
        println!("filter1");
        let filter = doc! {"searcher_key": searcher_key};
        println!("filter");
        let mut cursor = self
            .col
            .find(filter, None)
            .await
            .ok()
            .expect("Error pull btxns with searcher");
        let mut bundle_txns: Vec<BundledTransaction> = Vec::new();

        while let Some(txn) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            // println!("key: {:?}", txn.searcher_key);
            bundle_txns.push(txn)
        }
        // let stream = cursor.into_stream();
        Ok(bundle_txns)
    }
}
