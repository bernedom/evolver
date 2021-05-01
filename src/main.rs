use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Row, Table};

use tui::style::{Color, Style};
use tui::Terminal;

const WORLD_WIDTH: usize = 50;
const WORLD_HEIGHT: usize = 20;
fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let organisms = vec![vec!["X"; WORLD_WIDTH]; WORLD_HEIGHT];
    //let world = vec!["X"; WORLD_WIDTH];

    let return_value = terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(f.size());
        let world = Table::new(vec![
            // Row can be created from simple strings.
            Row::new(organisms[0].clone()),
            // You can style the entire row.
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        ])
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("world").borders(Borders::ALL))
        .widths(&[Constraint::Length(1); WORLD_WIDTH])
        .column_spacing(0);
        f.render_widget(world, chunks[0]);
        let log = Block::default().title("log").borders(Borders::ALL);
        f.render_widget(log, chunks[1]);
    });

    // for line in world {
    //     for field in line {
    //         println!("{}", field);
    //     }
    //     println!("End\n");
    // }
    return_value
}
