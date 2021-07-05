use std::collections::HashMap;
use std::io;

use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap};

use tui::style::{Color, Style};
use tui::Terminal;

pub const WORLD_WIDTH: usize = 50;
pub const WORLD_HEIGHT: usize = 20;

use crate::world::World;

pub fn draw<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    world: &World,
    genome_count: &HashMap<String, u16>,
    log_messages: &String,
) -> std::result::Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        let rows = world.chunks(WORLD_WIDTH).map(|item| {
            let cells = item.iter().map(|c| Cell::from(c.genome.as_str()));
            Row::new(cells).style(Style::default().fg(Color::Blue))
        });
        let world = Table::new(rows)
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("world").borders(Borders::ALL))
            .widths(&[Constraint::Length(1); WORLD_WIDTH])
            .column_spacing(0);
        f.render_widget(world, chunks[0]);
        let status_widget_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);
        let mut known_genomes_msg: String = "known genomes:\n".to_string();
        for (genome, count) in genome_count {
            known_genomes_msg += &(genome.to_owned() + ": " + &count.to_string() + "\n").to_owned();
        }
        let status = Paragraph::new(known_genomes_msg)
            .block(Block::default().title("status").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(status, status_widget_layout[0]);
        let log = Paragraph::new(log_messages.as_str())
            .block(Block::default().title("log").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(log, status_widget_layout[1]);
    })
}
