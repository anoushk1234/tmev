use crate::{arb_feed::*, get_and_parse_arb_feed};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use spinners::{Spinner, Spinners};
use std::sync::Arc;
use std::{
    error::Error,
    io::{self, Stdout},
    os::unix::thread,
    // thread::sleep,
    time::Duration,
};
use std::{marker::Send, vec};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_util::task::LocalPoolHandle;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap},
    Frame, Terminal,
};

pub struct App<'a> {
    state: TableState,
    title: &'a str,
    tabs: TabsState<'a>,
    items: Vec<Vec<String>>,
}
// unsafe impl Send for App {}
// unsafe impl Sync for App {}
impl<'a> App<'a> {
    pub fn new(title: &'a str, rows: Vec<Vec<String>>) -> App<'a> {
        App {
            title,
            state: TableState::default(),
            items: rows,
            tabs: TabsState::new(vec!["arbs", "bundles"]),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn go_to_explorer(&mut self) {
        let row_index = self.state.selected().unwrap();
        let row = self.items.get(row_index).unwrap();
        let mut explorer = "https://explorer.solana.com/tx/".to_string().to_owned();
        explorer.push_str(row.get(2).unwrap());

        open::that(explorer).unwrap();
    }
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }
}
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
        // else {
        //     self.index = self.titles.len() - 1;
        // }
    }
}

pub async fn display_table<'a>(
    rows: Vec<Vec<String>>,
) -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("JITO SEARCHER TERMINAL 🤑", rows);
    let res = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(terminal)
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![Spans::from("quit - [q] | reload - [r] | tab-change - [↔]")];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Legend",
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
async fn run_app<'a, B: Backend + std::marker::Send>(
    terminal: &mut Terminal<B>,
    mut app: App<'a>,
) -> io::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<Vec<String>>>(9000);
    let local_set = LocalPoolHandle::new(1);
    // let tx2 = tx.clone();
    // println!("enter");
    local_set.spawn_pinned(async move || loop {
        // panic!("fuck");
        sleep(Duration::from_millis(5000)).await;
        // let mut sp = Spinner::new(Spinners::Dots8Bit, " updating".into());
        let resp = get_and_parse_arb_feed().await.unwrap();
        tx.send(resp).await.unwrap();
        // sp.stop();
    });
    // let mut newitems: Vec<Vec<String>> = Vec::new();
    // newitems = app.items.clone();
    Ok(loop {
        // app.items = newitems.clone();
        terminal.draw(|f| draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Right => app.on_right(),
                KeyCode::Left => app.on_left(),
                KeyCode::Char('r') => {
                    terminal.clear();
                    sleep(Duration::from_millis(500));
                    continue;
                }
                KeyCode::Enter => app.go_to_explorer(),
                _ => {
                    if let Some(mut msg) = rx.recv().await {
                        if msg.len() > 0 {
                            // terminal.clear();
                            // msg.push(vec![
                            //     "hello1".to_string(),
                            //     "2".to_string(),
                            //     "3".to_string(),
                            //     "4".to_string(),
                            // ]);
                            // newitems = vec![vec![String::from("something")]];
                            continue;
                            // app.state.select(index)
                            // println!("reaches here");
                        }
                    }
                    // continue;
                }
            }
        }
        // Possibly add updating here?
    })
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::Green))
        .select(app.tabs.index);
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(20),
                Constraint::Length(3),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(area);

    ui(f, app, chunks[0]);
    draw_text(f, chunks[1]);
}
fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(20),
                Constraint::Length(3),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(area);

    ui(f, app, chunks[0]);
    // draw_text(f, chunks[1]);
}
//draws our table
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // println!("redraw :{:?}", app.items.len());
    // let rects = Layout::default()
    //     .constraints([Constraint::Percentage(70)].as_ref())
    //     .margin(2)
    //     .split(area);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightGreen);
    let header_cells = [
        "blocktime",
        "slot_id",
        "txn_hash",
        "profit_amt",
        "currency",
        "signer",
        "price_usd",
        "profit_usd",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Latest Arbs"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Percentage(20),
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Percentage(10),
            Constraint::Length(20),
            // Constraint::Min(10),
        ])
        .column_spacing(1);
    f.render_stateful_widget(t, area, &mut app.state);
}
