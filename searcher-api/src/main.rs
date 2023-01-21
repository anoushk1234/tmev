// mod api;
// mod models;
// mod repository;
use dotenv::dotenv;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
// use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
// use api::*;
// use repository::{BundleRepo, BundledTransactionRepo};
use std::env;
use thiserror::Error;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{self, channel};
// use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use tmev_protos::tmev_proto::bundle_service_server::{BundleService, BundleServiceServer};
use tmev_protos::tmev_proto::{Bundle, SubscribeBundlesRequest, SubscribeBundlesResponse};

const TIP_PROGRAM_KEY: &'static str = "T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt";
// mod tmev;
#[derive(Default)]
pub struct MevBundleClient {}
type ResponseStream =
    std::pin::Pin<Box<dyn Stream<Item = Result<SubscribeBundlesResponse, Status>> + Send>>;
type EchoResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl BundleService for MevBundleClient {
    type SubscribeBundlesStream = ResponseStream;

    async fn subscribe_bundles(
        &self,
        request: Request<SubscribeBundlesRequest>,
    ) -> EchoResult<Self::SubscribeBundlesStream> {
        // creating a queue or channel
        // let (mut send_stream_tx, rx) = channel(4);
        let (block_sender, mut block_receiver) = channel(100);
        let (mut block_update_sender, mut block_update_receiver) = channel(100);
        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect

        let (mut tx, rx) = mpsc::channel(128);
        let rpc_pub_sub = env::var("RPC_PUB_SUB").unwrap();
        tokio::spawn(block_subscribe_loop(rpc_pub_sub, block_sender));
        tokio::spawn(async move {
            // looping and sending our response using stream
            loop {
                // println!("here in");
                let rec = block_receiver.recv().await;
                // println!("here out {:?}", rec);
                match rec {
                    Some(res) => {
                        if let Some(block) = res.value.block {
                            println!("block here ");

                            if let Some(ref blk_txns) = block.transactions {
                                for (i, tx) in blk_txns.into_iter().enumerate() {
                                    let block_txns = block.transactions.clone();
                                    // println!("gets block txns");
                                    match &tx.transaction {
                                        EncodedTransaction::Json(inner_tx) => match &inner_tx
                                            .message
                                        {
                                            UiMessage::Raw(message) => {
                                                // let acc_keys = message
                                                //     .account_keys
                                                //     .clone()
                                                //     .into_iter()
                                                //     .map(|k| k)
                                                //     .collect::<Vec<String>>();
                                                // let ix   = message.instructions.clone().into_iter().map(|i| i).collect::<Vec<UiCompiledInstruction>>();
                                                let log_messages =
                                                    tx.meta.clone().unwrap().log_messages.unwrap();
                                                // println!("checking condition");
                                                let is_tip = log_messages.iter().any(|x| {
                                                    return x.contains(TIP_PROGRAM_KEY);
                                                });
                                                // for log in log_messages.iter().rev() {
                                                //     // if !log.contains(TIP_PROGRAM_KEY) {
                                                //     println!("not tip {}", log);
                                                //     // }
                                                //     tokio::time::sleep(
                                                //         tokio::time::Duration::from_millis(100),
                                                //     )
                                                //     .await;
                                                // }
                                                // println!("is tip {}", is_tip);
                                                if is_tip {
                                                    println!("found tip");
                                                    let picked_bundle =
                                                        get_picked_bundle(block_txns.clone(), i);
                                                    for bund in picked_bundle {
                                                        block_update_sender
                                                            .send(SubscribeBundlesResponse {
                                                                bundle: Some(Bundle {
                                                                    uuid: Uuid::new_v4()
                                                                        .to_string(),
                                                                    transaction_hash:
                                                                        tx_to_tx_hash(
                                                                            bund.transaction
                                                                                .clone(),
                                                                        )
                                                                        .unwrap(),
                                                                    searcher_key:
                                                                        find_searcher_key_from_tx(
                                                                            tx.transaction.clone(),
                                                                        )
                                                                        .unwrap_or(
                                                                            "no key".to_string(),
                                                                        ),
                                                                }),
                                                            })
                                                            .await
                                                            .unwrap();
                                                    }
                                                } else {
                                                    // println!("no tip");
                                                }
                                            }
                                            _ => println!("empty match"),
                                        },
                                        _ => println!("empty match outer"),
                                    }
                                }
                            }
                        } else {
                            println!("No block found {:?}", res);
                        }
                    }
                    None => {
                        // println!("No blockS found");
                        continue;
                    }
                }
                // tokio::time::sleep(Duration::from_secs(15)).await;
            }
        });

        tokio::spawn(async move {
            // let mut vec = Vec::new();
            // for i in 0..100 {
            //     vec.push(SubscribeBundlesResponse {
            //         bundle: Some(Bundle {
            //             uuid: Uuid::new_v4().to_string(),
            //             searcher_key: "searcher_key".to_string(),
            //             transaction_hash: "transaction_hash".to_string(),
            //         }),
            //     })
            // }
            loop {
                let block_update = block_update_receiver.recv().await;
                if let Some(update) = block_update {
                    let mut stream = Box::pin(
                        tokio_stream::iter(vec![update].into_iter())
                            .throttle(Duration::from_millis(200)),
                    );
                    while let Some(item) = stream.next().await {
                        match tx.send(Result::<_, Status>::Ok(item)).await {
                            Ok(_) => {}
                            Err(_item) => {
                                // output_stream was build from rx and both are dropped
                                break;
                            }
                        }
                    }
                    println!("\tclient disconnected");
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::SubscribeBundlesStream
        ))
    }
}

// pub fn create_data_stream() {}
pub fn tx_to_tx_hash(tx: EncodedTransaction) -> Option<String> {
    if let EncodedTransaction::Json(json_tx) = tx {
        return Some(json_tx.signatures.get(0).unwrap().to_string());
    } else {
        None
    }
}
pub fn get_picked_bundle(
    txs: Option<Vec<EncodedTransactionWithStatusMeta>>,
    i: usize,
) -> Vec<EncodedTransactionWithStatusMeta> {
    if i > 0 {
        return txs.unwrap().iter().as_slice()[(i - 4)..i].to_vec();
    } else {
        return txs.unwrap().iter().as_slice()[0..4].to_vec();
    }
}

pub fn find_searcher_key_from_tx(tx: EncodedTransaction) -> Option<String> {
    // pass tip tx only
    if let EncodedTransaction::Json(json_tx) = tx {
        if let UiMessage::Parsed(parsed_msg) = json_tx.message {
            return Some(parsed_msg.account_keys.get(0).unwrap().pubkey.clone());
        } else if let UiMessage::Parsed(parsed_msg) = json_tx.message {
            return Some(parsed_msg.account_keys.get(0).unwrap().pubkey.clone());
        } else {
            println!("not parsed msg {:?}", json_tx.message);
            return None;
        }
    } else {
        return None;
    }
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().json("gm")
// }
//std::io::Result<()>
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "1");
    env_logger::init();
    // let runtime = Builder::new_multi_thread().enable_all().build().unwrap();_

    // let db = BundleRepo::init().await;
    // let db_data = Data::new(db);
    // let new_db = BundledTransactionRepo::init().await?;
    // let new_db_data = Data::new(new_db);

    // let (slot_sender, slot_receiver) = channel(100);
    println!("before");
    println!("after");
    dotenv().ok();
    tokio::spawn(async move {
        let rpc_url = std::env::var("RPC_URL").expect("cant read env");
        // println!("here: {:?}", rpc_url);
        // slot_subscribe_loop(rpc_url.unwrap(), slot_sender);
    });

    // tokio::spawn(async move {

    let addr = "0.0.0.0:5005".parse().unwrap();
    let mev_client = MevBundleClient::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(BundleServiceServer::new(mev_client))
        .serve(addr)
        .await
        .ok()
        .expect("Error Starting Server");
    // });
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(db_data.clone())
    //         .service(create_bundle_stats)
    //         .service(get_all_bundle_stats)
    //         .app_data(new_db_data.clone())
    //         .service(create_bundled_transaction)
    //         .service(get_all_bundle_transactions)
    //         .service(get_bundled_transaction_by_searcher)
    //         .service(create_bundled_transactions)
    // })
    // .bind(("0.0.0.0", 8080))?
    // .run()
    // .await
}

// use actix_web::rt::time::sleep;
use futures::Stream;
use solana_client::client_error::ClientError;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::pubsub_client::PubsubClientError;
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_client::rpc_response;
use solana_client::rpc_response::{RpcBlockUpdate, SlotUpdate};
use solana_metrics::{datapoint_error, datapoint_info};
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_transaction_status::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, TransactionDetails, UiMessage,
    UiTransactionEncoding,
};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
// use tokio::time::sleep;
pub async fn block_subscribe_loop(
    pubsub_addr: String,
    mut block_sender: Sender<rpc_response::Response<RpcBlockUpdate>>,
) {
    let mut connect_errors: u64 = 0;
    let mut block_subscribe_errors: u64 = 0;
    let mut block_subscribe_disconnect_errors: u64 = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let req = PubsubClient::new(&pubsub_addr).await;
        println!("block_subscribe_loop");
        match req {
            Ok(pubsub_client) => match pubsub_client
                .block_subscribe(
                    RpcBlockSubscribeFilter::All,
                    Some(RpcBlockSubscribeConfig {
                        commitment: Some(CommitmentConfig {
                            commitment: CommitmentLevel::Confirmed,
                        }),
                        encoding: Some(UiTransactionEncoding::Json),
                        transaction_details: Some(TransactionDetails::Full),
                        show_rewards: Some(true),
                        max_supported_transaction_version: Some(0),
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
                        // println!("up: {:?}", block_update);
                        if block_sender.send(block_update).await.is_err() {
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
                    println!("block_subscribe_loop error: {:?}", e);
                    block_subscribe_errors += 1;
                    datapoint_error!(
                        "block_subscribe_error",
                        ("errors", block_subscribe_errors, i64),
                        ("error_str", e.to_string(), String),
                    );
                }
            },
            Err(e) => {
                println!("block_subscribe_loop error: {:?}", e);
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
        tokio::time::sleep(Duration::from_secs(1)).await;

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
