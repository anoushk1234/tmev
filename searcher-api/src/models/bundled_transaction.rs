use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BundledTransaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub searcher_key: String,
    // pub timestamp: i64,
    // pub slot: u64,
    pub transaction_hash: String,
    // pub balance_change: i64,
    // pub status: String,
}
