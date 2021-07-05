use crate::organism::Organism;
use std::collections::HashMap;

pub type World = Vec<Organism>;

pub fn insert_close_to_parent(organism: Organism, world: &mut World, idx: usize) {
    let first_dead = find_closest_dead_index(&world, idx);
    match first_dead {
        Ok(org) => {
            world[org] = organism;
        }
        Err(_msg) => {
            //log += "No space left on world, cannot spawn new organism";
        }
    }
}

fn find_closest_dead_index(world: &World, start_idx: usize) -> Result<usize, String> {
    if start_idx > world.len() {
        panic!("start_index out of bounds {}", world.len());
    }
    // return self if dead
    if !world[start_idx].is_alive() {
        return Ok(start_idx);
    }
    // search next dead cell forward
    let result_fwd = || -> Option<usize> {
        for i in start_idx..world.len() {
            if !world[i].is_alive() {
                return Some(i);
            }
        }
        None
    }();
    // search next dead cell backwards
    let result_backwd = || -> Option<usize> {
        for i in (0..start_idx).rev() {
            if !world[i].is_alive() {
                return Some(i);
            }
        }
        None
    }();

    if result_fwd == None && result_backwd == None {
        return Err("No space left in world".to_owned());
    }

    if result_fwd == None && result_backwd != None {
        return Ok(result_backwd.unwrap());
    }

    if result_fwd != None && result_backwd == None {
        return Ok(result_fwd.unwrap());
    }

    if result_fwd.unwrap() - start_idx <= start_idx - result_backwd.unwrap() {
        return Ok(result_fwd.unwrap());
    } else {
        return Ok(result_backwd.unwrap());
    }
}

pub fn count_genomes_map(
    world: &World,
    mut seed_map: HashMap<String, u16>,
) -> HashMap<String, u16> {
    for (_key, value) in seed_map.iter_mut() {
        *value = 0;
    }
    for organism in world.iter() {
        if !seed_map.contains_key(&organism.genome) && organism.is_alive() {
            seed_map.insert(organism.genome.clone(), 0);
        } else {
            let v = seed_map.get_mut(&organism.genome);
            match v {
                Some(v) => *v += 1,
                None => {}
            }
        }
    }
    return seed_map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_find_closest_fails_if_index_out_of_bounds() {
        let world: World = Vec::new();
        assert!(find_closest_dead_index(&world, 1).is_err());
    }

    #[test]
    fn test_find_closest_returns_err_if_no_dead_fields() {
        let mut world: World = Vec::with_capacity(3);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        assert!(find_closest_dead_index(&world, 1).is_err());
    }

    #[test]
    fn test_find_closest_returns_same_index_if_cell_is_dead() {
        let mut world: World = Vec::with_capacity(3);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        world[1].genome = "".to_owned();
        assert_eq!(find_closest_dead_index(&world, 1).unwrap(), 1);
    }

    #[test]
    fn test_find_next_dead_cell_if_only_one_is_free() {
        let mut world: World = Vec::with_capacity(3);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        let last_idx = world.len() - 1;
        world[last_idx].genome = "".to_owned();
        assert_eq!(find_closest_dead_index(&world, 1).unwrap(), last_idx);
    }
    #[test]
    fn test_find_closest_returns_dead_cell_if_only_one_is_free_and_dead_cell_is_lower_in_index() {
        let mut world: World = Vec::with_capacity(3);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        world[0].genome = "".to_owned();
        assert_eq!(find_closest_dead_index(&world, 1).unwrap(), 0);
    }

    #[test]
    fn test_closer_match_takes_precendence() {
        let mut world: World = Vec::with_capacity(6);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        world[5].genome = "".to_owned();
        world[2].genome = "".to_owned();
        assert_eq!(find_closest_dead_index(&world, 3).unwrap(), 2);
    }

    #[test]
    fn test_on_same_distance_forward_match_takes_precendence() {
        let mut world: World = Vec::with_capacity(6);
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        world[4].genome = "".to_owned();
        world[2].genome = "".to_owned();
        assert_eq!(find_closest_dead_index(&world, 3).unwrap(), 4);
    }

    #[test]
    fn test_count_genomes() {
        let mut world: World = Vec::with_capacity(6);
        let mut genome_count: HashMap<String, u16> = HashMap::new();
        for _i in 0..world.capacity() {
            world.push(Organism::new("a".to_owned()));
        }
        genome_count = count_genomes_map(&world, genome_count);
        assert_eq!(genome_count[&"a".to_owned()], 5);
    }

    #[test]
    fn test_count_genomes_retains_saved_genomes_but_puts_them_to_0() {
        let world: World = World::new();
        let mut genome_count: HashMap<String, u16> = HashMap::new();
        genome_count.insert("a".to_owned(), 6);
        genome_count.insert("b".to_owned(), 199);
        genome_count = count_genomes_map(&world, genome_count);
        assert_eq!(genome_count[&"a".to_owned()], 0);
        assert_eq!(genome_count[&"b".to_owned()], 0);
    }

    #[test]
    fn test_ignore_empty_string_in_genome_count() {
        let mut world: World = Vec::with_capacity(6);
        let mut genome_count: HashMap<String, u16> = HashMap::new();
        for _i in 0..world.capacity() {
            world.push(Organism::new("".to_owned()));
        }
        genome_count = count_genomes_map(&world, genome_count);
        assert_eq!(genome_count.len(), 0);
    }
}
