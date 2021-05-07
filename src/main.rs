use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use rand::{thread_rng, Rng};

mod event_listener;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut rng = thread_rng();
    let mut rng_filler = || -> &str {
        match rng.gen_bool(1.0 / 3.0) {
            true => "X",
            false => "",
        }
    };

    let mut organisms: Vec<Vec<&str>> = (0..ui::WORLD_HEIGHT)
        .map(|_| (0..ui::WORLD_WIDTH).map(|_| rng_filler()).collect())
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
        match organisms[0][1] {
            "X" => organisms[0][1] = "Y",
            "Y" => organisms[0][1] = "X",
            _ => organisms[0][1] = "Z",
        }
    }
    Ok(())
}
