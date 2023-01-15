use clap::arg;
use clap::command;
use clap::ArgMatches;
use clap::Command;
use clap::Parser;
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

fn main() {
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
                println!("{}", a);
            }
            _ => {
                // overview ui
            }
        }
    }
}
