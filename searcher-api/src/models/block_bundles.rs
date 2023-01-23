use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockBundles {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub bundles: Vec<SingleBundle>,
}
// pardon the naming scheme too many "Bundles"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleBundle {
    pub searcher_key: String,
    pub uuid: String,
    pub transaction_hash: String,
    pub slot: String,
}
