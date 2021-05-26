use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use rand::Rng;

mod event_listener;
mod organism;
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

    let mut organisms: Vec<organism::Organism> = (0..ui::WORLD_HEIGHT * ui::WORLD_WIDTH)
        .map(|_| organism::Organism {
            genome: rng_filler(),
            ..Default::default()
        })
        .collect();

    let mut log = String::from("");

    // set up input handling
    let rx = event_listener::spawn_event_listener();

    loop {
        ui::draw(&mut terminal, &organisms, &log)?;

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

        organisms.iter_mut().for_each(|i| {
            i.age += 1;
            // todo parameterize conditions and probabilities  for multiplication, death etc.
            let max_age: f64 = 100.0;
            // organisms die faster with age
            match rng.gen_bool(i.age as f64 / max_age) {
                true => {
                    i.genome = String::from("");
                    i.age = 0
                }
                false => {}
            }
            // organisms are reborn after a cooldown period
            // todo change this so existing organisms are rewspawning from their own genomes
            if i.age > 10 && !i.is_alive() {
                match rng.gen_bool(i.age as f64 / (max_age * 2.0)) {
                    true => {
                        let mutation_probability = 1.0 / 1000.0;
                        match rng.gen_bool(mutation_probability) {
                            true => {
                                i.genome =
                                    String::from(GENOMES[rng.gen_range(0..GENOMES.len())] as char);
                                log += format!("New genome {} from mutation\n", i.genome.as_str())
                                    .as_str();
                            }
                            false => i.genome = String::from(initial_genome),
                        }
                        i.age = 0;
                    }
                    false => {}
                }
            }
        });

        // todo limit to 60fps
    }
    Ok(())
}
