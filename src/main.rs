use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use rand::Rng;

mod event_listener;
mod ui;

const GENOMES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // populate
    let mut rng = rand::thread_rng();

    let initial_genome = GENOMES[rng.gen_range(0..GENOMES.len())] as char;

    let mut rng_filler = || -> String {
        match rng.gen_bool(1.0 / 5.0) {
            true => String::from(initial_genome),
            false => String::from(""),
        }
    };

    let mut organisms: Vec<String> = (0..ui::WORLD_HEIGHT * ui::WORLD_WIDTH)
        .map(|_| rng_filler())
        .collect();

    // set up input handling
    let rx = event_listener::spawn_event_listener();

    loop {
        ui::draw(&mut terminal, &organisms)?;

        match rx.recv()? {
            event_listener::Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            event_listener::Event::Tick => {}
        }

        // todo put logic in here, limit to 60fps
        // todo add event handling to quit qith esc or 'q'
        match organisms[0].as_str() {
            "X" => organisms[0] = String::from("Y"),
            "Y" => organisms[0] = String::from("X"),
            _ => organisms[0] = String::from("Z"),
        }
    }
    Ok(())
}
