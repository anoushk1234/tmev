use std::thread;
use std::time::Duration;

use crate::arb_feed::ArbFeedResponse;
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
                loop {
                    let mut sp = Spinner::new(Spinners::Dots8Bit, " loading arbs 🥩".into());
                    let feed = arb_feed::get_arb_feed().await;
                    let parsed = serde_json::from_str::<ArbFeedResponse>(
                        feed.unwrap().text().await.unwrap().as_str(),
                    )
                    .unwrap()
                    .query_data;
                    let block_times = parsed
                        .block_time
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>();
                    let slot_ids = parsed
                        .slot_id
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>();
                    let transaction_hashes = parsed
                        .transaction_hash
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>();

                    let profit_amts_string = parsed
                        .profit_amount
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>();
                    let profit_amts = profit_amts_string
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>();
                    let currencies = parsed
                        .currency
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<&str>>();
                    let signers = parsed
                        .signers
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>();
                    let prices_usd = parsed
                        .price_usd
                        .iter()
                        .map(|f| match f.as_str() {
                            Some(f) => f,
                            None => "None",
                        })
                        .collect::<Vec<&str>>();
                    let profits_usd = parsed
                        .price_usd
                        .iter()
                        .map(|f| match f.as_str() {
                            Some(f) => f,
                            None => "None",
                        })
                        .collect::<Vec<&str>>();
                    let mut row_vec = Vec::new();

                    for index in 0..transaction_hashes.len() {
                        let row = vec![
                            block_times[index],
                            slot_ids[index],
                            transaction_hashes[index],
                            profit_amts[index],
                            currencies[index],
                            signers[index],
                            prices_usd[index],
                            profit_amts[index],
                        ];
                        row_vec.push(row);
                    }

                    // sleep(Duration::from_secs(3));
                    sp.stop();

                    let mut terminal = display_table(row_vec).unwrap();
                    // thread::sleep(Duration::from_millis(5000));
                }
            }
            _ => {
                // overview ui
            }
        }
    }
}
