use std::error::Error;

use reqwest::Response;

pub async fn get_arb_feed() -> Result<Response, Box<dyn Error>> {
    let url = "https://jito.retool.com/api/public/7e37389a-c991-4fb3-a3cd-b387859c7da1/query?queryName=arb_feed";
    let body = "{\"userParams\":{\"queryParams\":{\"length\":0},\"databaseNameOverrideParams\":{\"length\":0},\"databaseHostOverrideParams\":{\"length\":0},\"databaseUsernameOverrideParams\":{\"length\":0},\"databasePasswordOverrideParams\":{\"length\":0}},\"password\":\"\",\"environment\":\"production\",\"queryType\":\"SqlQueryUnified\",\"frontendVersion\":\"1\",\"releaseVersion\":null,\"includeQueryExecutionMetadata\":true}" ;
    let parsed = serde_json::from_str::<RequestBody>(body).unwrap();
    let client = reqwest::Client::new();
    let resp = client.post(url).json(&parsed).send().await?;
    // println!("{:#?}", resp.text().await);
    Ok(resp)
}

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub user_params: UserParams,
    pub password: String,
    pub environment: String,
    pub query_type: String,
    pub frontend_version: String,
    pub release_version: Value,
    pub include_query_execution_metadata: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    pub query_params: QueryParams,
    pub database_name_override_params: DatabaseNameOverrideParams,
    pub database_host_override_params: DatabaseHostOverrideParams,
    pub database_username_override_params: DatabaseUsernameOverrideParams,
    pub database_password_override_params: DatabasePasswordOverrideParams,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseNameOverrideParams {
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseHostOverrideParams {
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsernameOverrideParams {
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabasePasswordOverrideParams {
    pub length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbFeedResponse {
    #[serde(rename = "__retoolWrappedQuery__")]
    pub retool_wrapped_query: bool,
    pub query_data: QueryData,
    pub query_execution_metadata: QueryExecutionMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryData {
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: Vec<String>,
    #[serde(rename = "profit_amount")]
    pub profit_amount: Vec<f64>,
    pub signers: Vec<String>,
    pub currency: Vec<String>,
    #[serde(rename = "slot_id")]
    pub slot_id: Vec<String>,
    #[serde(rename = "block_time")]
    pub block_time: Vec<String>,
    #[serde(rename = "price_usd")]
    pub price_usd: Vec<Value>,
    #[serde(rename = "profit_usd")]
    pub profit_usd: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExecutionMetadata {
    pub estimated_response_size_bytes: i64,
    pub resource_time_taken_ms: i64,
    pub is_preview: bool,
    pub resource_type: String,
    pub last_received_from_resource_at: i64,
}
