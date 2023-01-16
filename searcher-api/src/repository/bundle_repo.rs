use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;

use crate::models::BundleStat;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection,
};

pub struct BundleRepo {
    col: Collection<BundleStat>,
}

impl BundleRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("data");
        let col: Collection<BundleStat> = db.collection("BundleStat");
        // MongoRepo { col }
        BundleRepo { col }
    }
    pub async fn create_bundle_in_db(
        &self,
        new_bundle: BundleStat,
    ) -> Result<InsertOneResult, Error> {
        let new_doc = BundleStat {
            id: None,
            ..new_bundle
        };
        let bundle_stat = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating bundle in db");
        Ok(bundle_stat)
    }
    pub async fn get_bundles_from_db(&self) -> Result<Vec<BundleStat>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut bundle_stats: Vec<BundleStat> = Vec::new();
        while let Some(bundle) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            bundle_stats.push(bundle)
        }
        Ok(bundle_stats)
    }
}
