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

fn find_closest_empty_index(world: &Vec<Organism>, start_idx: usize) -> Result<usize, String>
{
    if start_idx > world.len(){
        panic!("start_index out of bounds {}", world.len());
    }
    if start_idx == 1{
        Err("No space left in world".to_owned())
    }
    else{
        Ok(0)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_find_closest_fails_if_index_out_of_bounds()
    {
        let world : Vec<Organism> = Vec::new(); 
        assert!(find_closest_empty_index(&world, 1).is_err());
    }

    #[test]
    fn test_find_closest_returns_err_if_no_empty_fields()
    {
        let mut world : Vec<Organism> = Vec::with_capacity(3);
        for i in 0..world.capacity()
        {
            world.push(Organism::new("a".to_owned()));
        }
        
        assert!(find_closest_empty_index(&world, 1).is_err());
    }
}
