mod api;
mod models;
mod repository;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::*;
use repository::{BundleRepo, BundledTransactionRepo};
use std::env;
use thiserror::Error;
use tokio::sync::mpsc::{self, channel, Receiver};

use tonic::{transport::Server, Request, Response, Status};

use tmev::bundle_service_server::{BundleService, BundleServiceServer};
use tmev::{SubscribeBundlesRequest, SubscribeBundlesResponse};
mod tmev;
#[derive(Default)]
pub struct MevBundleClient {}

#[tonic::async_trait]
impl BundleService for MevBundleClient {
    type SubscribeBundlesStream = mpsc::Receiver<Result<SubscribeBundlesResponse, Status>>;
    async fn subscribe_bundles(
        &self,
        request: Request<SubscribeBundlesRequest>,
    ) -> Result<Response<Self::SubscribeBundlesStream>, Status> {
        // creating a queue or channel
        let (mut tx, rx) = channel(4);
        // creating a new task
        tokio::spawn(async move {
            // looping and sending our response using stream
            for _ in 0..4 {
                // sending response to our channel
                tx.send(Ok(SubscribeBundlesResponse { bundle: None })).await;
            }
        });
        // returning our reciever so that tonic can listen on reciever and send the response to client
        Ok(Response::new(rx))
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("gm")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "1");
    env_logger::init();

    let db = BundleRepo::init().await;
    let db_data = Data::new(db);
    let new_db = BundledTransactionRepo::init().await;
    let new_db_data = Data::new(new_db);
    let (block_sender, block_receiver) = channel(100);
    let (slot_sender, slot_receiver) = channel(100);
    //
    tokio::spawn(async move {
        let rpc_url = env::var("RPC_URL").unwrap();
        block_subscribe_loop(rpc_url, block_sender);

        // let resp = block_receiver
        //     .recv()
        //     .await
        //     .unwrap_or_else(BundleSubscribeError::Shutdown);
    });
    //
    tokio::spawn(async move {
        let rpc_url = env::var("RPC_URL").unwrap();
        slot_subscribe_loop(rpc_url, slot_sender);
    });

    tokio::spawn(async move {
        let addr = "[::1]:50051".parse().unwrap();
        let say = MevBundleClient::default();
        println!("Server listening on {}", addr);
        Server::builder()
            .add_service(BundleServiceServer::new(say))
            .serve(addr)
            .await?;
        Ok(())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_bundle_stats)
            .service(get_all_bundle_stats)
            .app_data(new_db_data.clone())
            .service(create_bundled_transaction)
            .service(get_all_bundle_transactions)
            .service(get_bundled_transaction_by_searcher)
            .service(create_bundled_transactions)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

use futures::StreamExt;
use solana_client::client_error::ClientError;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::pubsub_client::PubsubClientError;
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_client::rpc_response;
use solana_client::rpc_response::{RpcBlockUpdate, SlotUpdate};
use solana_metrics::{datapoint_error, datapoint_info};
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::delay_for;

pub async fn block_subscribe_loop(
    pubsub_addr: String,
    mut block_receiver: Sender<rpc_response::Response<RpcBlockUpdate>>,
) {
    let mut connect_errors: u64 = 0;
    let mut block_subscribe_errors: u64 = 0;
    let mut block_subscribe_disconnect_errors: u64 = 0;

    loop {
        delay_for(Duration::from_secs(1)).await;

        match PubsubClient::new(&pubsub_addr).await {
            Ok(pubsub_client) => match pubsub_client
                .block_subscribe(
                    RpcBlockSubscribeFilter::All,
                    Some(RpcBlockSubscribeConfig {
                        commitment: Some(CommitmentConfig {
                            commitment: CommitmentLevel::Confirmed,
                        }),
                        encoding: Some(UiTransactionEncoding::Base64),
                        transaction_details: Some(TransactionDetails::Signatures),
                        show_rewards: Some(true),
                        max_supported_transaction_version: None,
                    }),
                )
                .await
            {
                Ok((mut block_update_subscription, _unsubscribe_fn)) => {
                    while let Some(block_update) = block_update_subscription.next().await {
                        datapoint_info!(
                            "block_subscribe_slot",
                            ("slot", block_update.context.slot, i64)
                        );
                        if block_receiver.send(block_update).await.is_err() {
                            datapoint_error!("block_subscribe_send_error", ("errors", 1, i64));
                            return;
                        }
                    }
                    block_subscribe_disconnect_errors += 1;
                    datapoint_error!(
                        "block_subscribe_disconnect_error",
                        ("errors", block_subscribe_disconnect_errors, i64)
                    );
                }
                Err(e) => {
                    block_subscribe_errors += 1;
                    datapoint_error!(
                        "block_subscribe_error",
                        ("errors", block_subscribe_errors, i64),
                        ("error_str", e.to_string(), String),
                    );
                }
            },
            Err(e) => {
                connect_errors += 1;
                datapoint_error!(
                    "block_subscribe_pubsub_connect_error",
                    ("errors", connect_errors, i64),
                    ("error_str", e.to_string(), String)
                );
            }
        }
    }
}

pub async fn slot_subscribe_loop(pubsub_addr: String, mut slot_sender: Sender<Slot>) {
    let mut connect_errors: u64 = 0;
    let mut slot_subscribe_errors: u64 = 0;
    let mut slot_subscribe_disconnect_errors: u64 = 0;

    loop {
        delay_for(Duration::from_secs(1)).await;

        match PubsubClient::new(&pubsub_addr).await {
            Ok(pubsub_client) => match pubsub_client.slot_updates_subscribe().await {
                Ok((mut slot_update_subscription, _unsubscribe_fn)) => {
                    while let Some(slot_update) = slot_update_subscription.next().await {
                        match slot_update {
                            SlotUpdate::FirstShredReceived { slot, timestamp: _ } => {
                                datapoint_info!("slot_subscribe_slot", ("slot", slot, i64));
                                if slot_sender.send(slot).await.is_err() {
                                    datapoint_error!(
                                        "slot_subscribe_send_error",
                                        ("errors", 1, i64)
                                    );
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }
                    slot_subscribe_disconnect_errors += 1;
                    datapoint_error!(
                        "slot_subscribe_disconnect_error",
                        ("errors", slot_subscribe_disconnect_errors, i64)
                    );
                }
                Err(e) => {
                    slot_subscribe_errors += 1;
                    datapoint_error!(
                        "slot_subscribe_error",
                        ("errors", slot_subscribe_errors, i64),
                        ("error_str", e.to_string(), String),
                    );
                }
            },
            Err(e) => {
                connect_errors += 1;
                datapoint_error!(
                    "slot_subscribe_pubsub_connect_error",
                    ("errors", connect_errors, i64),
                    ("error_str", e.to_string(), String)
                );
            }
        }
    }
}

#[derive(Debug, Error)]
enum BundleSubscribeError {
    #[error("TonicError {0}")]
    TonicError(#[from] tonic::transport::Error),
    #[error("GrpcError {0}")]
    GrpcError(#[from] Status),
    #[error("RpcError {0}")]
    RpcError(#[from] ClientError),
    #[error("PubSubError {0}")]
    PubSubError(#[from] PubsubClientError),
    // #[error("BlockEngineConnectionError {0}")]
    // BlockEngineConnectionError(#[from] BlockEngineConnectionError),
    #[error("Shutdown")]
    Shutdown,
}
