use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleStat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub min_bundle_send_slot: u64,
    pub max_bundle_send_slot: u64,
    pub bundles_landed: i64,
    pub num_bundles_dropped: i64,
    pub mempool_txs_landed_no_bundle: i64,
}
