extern crate dotenv;
use std::env;

use dotenv::dotenv;
use futures::{future::IntoStream, TryStreamExt};
use log::info;

use crate::models::BlockBundles;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertManyResult, InsertOneResult},
    Client, Collection, Cursor,
};
// use env_logger::

pub struct BlockBundlesRepo {
    col: Collection<BlockBundles>,
}

impl BlockBundlesRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("data");
        let col: Collection<BlockBundles> = db.collection("BlockBundles");
        // MongoRepo { col }
        BlockBundlesRepo { col }
    }
    pub async fn create_one(&self, data: BlockBundles) -> Result<InsertOneResult, Error> {
        let new_doc = BlockBundles { id: None, ..data };
        let result = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating bundle in db");
        Ok(result)
    }
    pub async fn create_many(
        &self,
        mut data: Vec<BlockBundles>,
    ) -> Result<InsertManyResult, Error> {
        let new_docs = data
            .iter_mut()
            .map(move |f| BlockBundles {
                id: None,
                ..f.clone()
            })
            .collect::<Vec<BlockBundles>>();
        let result = self
            .col
            .insert_many(new_docs, None)
            .await
            .ok()
            .expect("Error creating bundled txn in db");
        Ok(result)
    }
    pub async fn get_all(&self) -> Result<Vec<BlockBundles>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of btxns");
        let mut bundle_txns: Vec<BlockBundles> = Vec::new();
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
    // pub async fn get_all_by_searcher(&self, searcher_key: &String) -> Result<BlockBundles, Error> {
    //     println!("searcher_key1: {:?}", searcher_key);
    //     // let obj_id = ObjectId::parse_str("kash.sol").unwrap();
    //     println!("filter1");
    //     let filter = doc! {"bundles": searcher_key};
    //     println!("filter");
    //     let mut cursor = self
    //         .col
    //         .find(filter, None)
    //         .await
    //         .ok()
    //         .expect("Error pull btxns with searcher");
    //     let mut response_data: BlockBundles = BlockBundles {
    //         id: None,
    //         bundles: Vec::new(),
    //     };

    //     while let Some(block_bundles) = cursor
    //         .try_next()
    //         .await
    //         .ok()
    //         .expect("Error mapping through cursor")
    //     {
    //         // println!("key: {:?}", txn.searcher_key);
    //         for item in block_bundles.bundles{

    //         }
    //     }
    //     // let stream = cursor.into_stream();
    //     Ok(bundle_txns)
    // }
}
