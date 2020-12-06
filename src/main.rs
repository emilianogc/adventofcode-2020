#![feature(str_split_once)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::io::{Write, stdout};
use std::{env, io, thread};
use text_io::read;
use termion::raw::IntoRawMode;
use tui::backend::{TermionBackend, Backend};
use tui::Terminal;
use std::error::Error;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{ListItem, List, Block, Borders, ListState};
use tui::text::{Span, Spans};
use tui::style::{Modifier, Style, Color};
use termion::event::Key;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use termion::input::TermRead;


/*fn main() {
    let arg = env::args().nth(1).and_then(|v| v.parse::<u8>().ok());
    let selection = match arg {
        Some(selection) => selection,
        _ => {
            println!();
            println!();
            println!(
                "1.                   . {}{}{} .                    ",
                "....".forest_green(),
                "|".bark_brown(),
                "....".forest_green()
            );
            println!("2.");
            println!("3.");

            print!("\nSelection: ");
            io::stdout().flush().unwrap();
            let read: String = read!();
            read.parse::<u8>().unwrap()
        }
    };

    match selection {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        _ => eprintln!("Not a valid option"),
    }
}
*/
fn main() {
    let mut terminal = stdout().into_raw_mode()
        .map(MouseTerminal::from)
        .map(AlternateScreen::from)
        .map(TermionBackend::new)
        .and_then(Terminal::new)
        .unwrap();

    let events = Events::new();
    let mut app = App::new();

    let selected = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)].as_ref())
                .split(f.size());

            let rect = Layout::default().
                direction(Direction::Vertical)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)].as_ref())
                .split(chunks[1])[1];

            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(i.0)];
                    ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
                })
                .collect();

            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::REVERSED),
                );

            f.render_stateful_widget(items, rect, &mut app.items.state);
        }).unwrap();

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break None;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Up => {
                    app.items.previous();
                }
                Key::Char('\n') => {
                    break app.items.state.selected();
                }
                _ => {}
            },
        }
    };

    terminal.backend_mut();

    match selected {
        Some(1) => day1::main(),
        Some(2) => day2::main(),
        Some(3) => day3::main(),
        Some(4) => day4::main(),
        Some(5) => day5::main(),
        _ => eprintln!("Not a valid option")
    }
}

struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Day 1", 1),
                ("Day 2", 2),
                ("Day 3", 1),
                ("Day 4", 3),
                ("Day 5", 1),
            ]),
        }
    }

}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub enum Event<I> {
    Input(I),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    ignore_exit_key: Arc<AtomicBool>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}



/*trait CustomColors {
    fn bark_brown(self) -> ColoredString
    where
        Self: Sized + Colorize,
    {
        self.truecolor(140, 88, 80)
    }

    fn forest_green(self) -> ColoredString
    where
        Self: Sized + Colorize,
    {
        self.truecolor(34, 139, 34)
    }
}*/

// impl<'a> CustomColors for &'a str {}
