use std::io;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, Row, Table};

use tui::style::{Color, Style};
use tui::Terminal;

pub const WORLD_WIDTH: usize = 50;
pub const WORLD_HEIGHT: usize = 20;

pub fn draw<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    organisms: &Vec<Vec<&str>>,
) -> std::result::Result<(), io::Error> {
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
    })
}
