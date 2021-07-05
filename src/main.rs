use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::collections::HashMap;

use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use rand::Rng;

mod event_listener;
mod organism;
mod ui;
mod world;

use organism::Organism;
use world::World;

const GENOMES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789!";

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

    let mut rng = rand::thread_rng();

    let initial_genome = GENOMES[rng.gen_range(0..GENOMES.len())] as char;

    let mut rng_filler = || -> String {
        match rng.gen_bool(1.0 / 5.0) {
            true => String::from(initial_genome),
            false => String::from(""),
        }
    };

    let mut world: World = (0..ui::WORLD_HEIGHT * ui::WORLD_WIDTH)
        .map(|_| Organism::new(rng_filler()))
        .collect();

    let log = String::from("");

    // set up input handling
    let rx = event_listener::spawn_event_listener();
    let mut genome_count: HashMap<String, u16> = HashMap::new();

    loop {
        genome_count = world::count_genomes(&world, genome_count);
        ui::draw(&mut terminal, &world, &genome_count, &log)?;

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

        for (idx, organism) in world.iter_mut().enumerate() {
            organism.age += 1;
            // organisms die faster with age
            match rng.gen_bool(organism.age as f64 / organism.max_age as f64) {
                true => {
                    organism.genome = String::from("");
                    organism.age = 0
                }
                false => {}
            }

            if organism.is_alive() {
                let new_organism = spawn(organism, &mut rng);
                match new_organism {
                    Some(org) => {
                        newborns.push((org, idx));
                    }
                    None => {}
                }
            }
        }

        while newborns.len() > 0 {
            if let Some(newborn) = newborns.pop() {
                world::insert_close_to_parent(newborn.0, &mut world, newborn.1);
            }
        }

        // todo limit to 60fps
    }
    Ok(())
}
