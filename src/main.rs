#![feature(async_closure)]
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::arb_feed::ArbFeedResponse;
use arb_feed::QueryData;
use clap::arg;
use clap::command;
use clap::ArgMatches;
use clap::Command;
use clap::Parser;
use spinners::{Spinner, Spinners};
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{io, thread, time::Duration};
// use tui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout, Margin},
//     style::{Color, Modifier, Style},
//     text::{Span, Spans},
//     widgets::{Block, Borders, Cell, Row, Table, Widget},
//     Terminal,
// };

/// Simple program to greet a person
// #[derive(Parser, Debug, Clone)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long)]
//     address: String,
// }
mod arb_feed;
mod arb_table;

use arb_table::*;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let cmd = Command::new("tmev").args(&[
        arg!(--address <ADDRESS> "An address to filter transactions by"),
        arg!([arbs] "View a table of the recent arbs in order of most profitable"),
    ]);

    let matches = cmd.get_matches();

    for arg in matches.ids().into_iter() {
        let a = arg.as_str();
        match a {
            "address" => {
                println!("{}", a);
            }
            "arbs" => {
                // println!("{}", a);

                let mut sp = Spinner::new(Spinners::Dots8Bit, " loading arbs 🥩".into());
                let feed = arb_feed::get_arb_feed().await;
                let parsed = serde_json::from_str::<ArbFeedResponse>(
                    feed.unwrap().text().await.unwrap().as_str(),
                )
                .unwrap()
                .query_data;
                // let block_times = parsed
                //     .block_time
                //     .iter()
                //     .map(|f| f.as_str())
                //     .collect::<Vec<&str>>();
                // let slot_ids = parsed
                //     .slot_id
                //     .iter()
                //     .map(|f| f.as_str())
                //     .collect::<Vec<&str>>();
                // let transaction_hashes = parsed
                //     .transaction_hash
                //     .iter()
                //     .map(|f| f.as_str())
                //     .collect::<Vec<&str>>();

                let profit_amts = parsed
                    .profit_amount
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>();
                // let profit_amts = profit_amts_string
                //     .iter()
                //     .map(|f| f.as_str())
                //     .collect::<Vec<&str>>();
                // let currencies = parsed
                //     .currency
                //     .iter()
                //     .map(|c| c.as_str())
                //     .collect::<Vec<&str>>();
                // let signers = parsed
                //     .signers
                //     .iter()
                //     .map(|f| f.as_str())
                //     .collect::<Vec<&str>>();
                let prices_usd = parsed
                    .price_usd
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                let profits_usd = parsed
                    .price_usd
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                let QueryData {
                    block_time,
                    slot_id,
                    transaction_hash,
                    profit_amount,
                    currency,
                    signers,
                    price_usd,
                    profit_usd,
                } = parsed;
                let mut row_vec = Vec::new();

                for index in 0..transaction_hash.len() {
                    let row: Vec<String> = vec![
                        block_time[index].clone(),
                        slot_id[index].clone(),
                        transaction_hash[index].clone(),
                        profit_amts[index].clone(),
                        currency[index].clone(),
                        signers[index].clone(),
                        prices_usd[index].clone(),
                        profits_usd[index].clone(),
                    ];
                    row_vec.push(row);
                }

                sp.stop();

                display_table(row_vec).await.unwrap();
            }

            _ => {
                // overview ui
            }
        }
    }
}

// type Something = Arc<tokio::sync::Mutex<Vec<Vec<String>>>>;
// impl Something for SomethingStruct {}
// type Res<T> = Result<T, dyn Error>;
pub async fn get_and_parse_arb_feed() -> Result<Vec<Vec<String>>, Box<dyn Error + std::marker::Send>>
{
    let feed = arb_feed::get_arb_feed().await;
    let parsed =
        serde_json::from_str::<ArbFeedResponse>(feed.unwrap().text().await.unwrap().as_str())
            .unwrap()
            .query_data;

    let profit_amts = parsed
        .profit_amount
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    let prices_usd = parsed
        .price_usd
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    let profits_usd = parsed
        .price_usd
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    let QueryData {
        block_time,
        slot_id,
        transaction_hash,
        profit_amount,
        currency,
        signers,
        price_usd,
        profit_usd,
    } = parsed;
    let mut row_vec = Vec::new();

    for index in 0..transaction_hash.len() {
        let row: Vec<String> = vec![
            block_time[index].clone(),
            slot_id[index].clone(),
            transaction_hash[index].clone(),
            profit_amts[index].clone(),
            currency[index].clone(),
            signers[index].clone(),
            prices_usd[index].clone(),
            profits_usd[index].clone(),
        ];
        row_vec.push(row);
    }
    Ok(row_vec)
}
