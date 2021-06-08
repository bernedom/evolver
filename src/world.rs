use crate::organism::Organism;

pub fn insert_close_to_parent(organism: Organism, world: &mut Vec<Organism>, idx: usize) {
    let first_dead = world.iter().position(|o| !o.is_alive());
    match first_dead {
        Some(org) => {
            world[org] = organism;
        }
        None => {
            //log += "No space left on world, cannot spawn new organism";
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
