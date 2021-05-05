use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, Row, Table};

use tui::style::{Color, Style};
use tui::Terminal;

const WORLD_WIDTH: usize = 50;
const WORLD_HEIGHT: usize = 20;
fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut organisms = vec![vec!["X"; WORLD_WIDTH]; WORLD_HEIGHT];

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
        // todo put logic in here, limit to 60fps
        // todo add event handling to quit qith esc or 'q'
        match organisms[0][1] {
            "X" => organisms[0][1] = "Y",
            "Y" => organisms[0][1] = "X",
            _ => {}
        }
    }
}
