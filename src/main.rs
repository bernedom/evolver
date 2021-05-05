use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, Row, Table};

use tui::style::{Color, Style};
use tui::Terminal;

const WORLD_WIDTH: usize = 50;
const WORLD_HEIGHT: usize = 20;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut organisms = vec![vec!["X"; WORLD_WIDTH]; WORLD_HEIGHT];

    // set up input handling
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let rows = organisms.iter().map(|item| {
                let cells = item.iter().map(|c| Cell::from(*c));
                Row::new(cells).style(Style::default().fg(Color::Blue))
            });
            let world = Table::new(rows)
                .style(Style::default().fg(Color::White))
                .block(Block::default().title("world").borders(Borders::ALL))
                .widths(&[Constraint::Length(1); WORLD_WIDTH])
                .column_spacing(0);
            f.render_widget(world, chunks[0]);
            let log = Block::default().title("log").borders(Borders::ALL);
            f.render_widget(log, chunks[1]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }

        // todo put logic in here, limit to 60fps
        // todo add event handling to quit qith esc or 'q'
        match organisms[0][1] {
            "X" => organisms[0][1] = "Y",
            "Y" => organisms[0][1] = "X",
            _ => {}
        }
    }
    Ok(())
}
