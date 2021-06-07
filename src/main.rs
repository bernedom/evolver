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
use organism::Organism;

const GENOMES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";

fn spawn(o: &Organism, rng: &mut rand::prelude::ThreadRng) -> Option<Organism> {
    if o.age > 10 {
        // todo store conditions in organism struct
        match rng.gen_bool(o.age as f64 / (o.max_age as f64 / 2.0)) {
            true => {
                let mut spawned = Organism::new(String::from(o.genome.as_str()));
                let mutation_probability = 1.0 / 1000.0;
                match rng.gen_bool(mutation_probability) {
                    true => {
                        spawned.genome =
                            String::from(GENOMES[rng.gen_range(0..GENOMES.len())] as char);
                        // log +=
                        //     format!("New genome '{}' from mutation\n", o.genome.as_str())
                        //         .as_str();
                    }
                    false => {}
                }
                return Some(spawned);
            }
            false => {}
        }
    }
    None
}

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

    let mut organisms: Vec<Organism> = (0..ui::WORLD_HEIGHT * ui::WORLD_WIDTH)
        .map(|_| Organism::new(rng_filler()))
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

        let mut newborns = Vec::new();

        for i in organisms.iter_mut() {
            i.age += 1;
            // organisms die faster with age
            match rng.gen_bool(i.age as f64 / i.max_age as f64) {
                true => {
                    i.genome = String::from("");
                    i.age = 0
                }
                false => {}
            }

            if i.is_alive() {
                let new_organism = spawn(i, &mut rng);
                match new_organism {
                    Some(org) => {
                        newborns.push(org);
                    }
                    None => {}
                }
            }
            // organisms are reborn after a cooldown period
            // todo change this so existing organisms are rewspawning from their own genomes
            if i.age > 10 && !i.is_alive() {}
        }

        // todo insert newborns close to parent
        while newborns.len() > 0 {
            if let Some(newborn) = newborns.pop() {
                let first_dead = organisms.iter().position(|o| !o.is_alive());
                match first_dead {
                    Some(org) => {
                        organisms[org] = newborn;
                    }
                    None => {
                        log += "No space left on world, cannot spawn new organism";
                    }
                }
            }
        }

        // todo limit to 60fps
    }
    Ok(())
}
