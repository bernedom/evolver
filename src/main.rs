use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let world = vec![vec![0; 50]; 20];

    let return_value = terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("evolver").borders(Borders::ALL);
        f.render_widget(block, size);
    });

    // for line in world {
    //     for field in line {
    //         println!("{}", field);
    //     }
    //     println!("End\n");
    // }
    return_value
}
