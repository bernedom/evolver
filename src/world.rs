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
    if !world[start_idx].is_alive()
    {
        return Ok(start_idx);
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
        for _i in 0..world.capacity()
        {
            world.push(Organism::new("a".to_owned()));
        }
        
        assert!(find_closest_empty_index(&world, 1).is_err());
    }


    #[test]
    fn test_find_closest_returns_same_index_if_cell_is_empty()
    {
        let mut world : Vec<Organism> = Vec::with_capacity(3);
        for _i in 0..world.capacity()
        {
            world.push(Organism::new("a".to_owned()));
        }
        world[1].genome = "".to_owned();
        
        assert_eq!(find_closest_empty_index(&world, 1).unwrap(), 1);
    }

    #[test]
    fn test_find_closest_returns_next_index_upwards_if_cell_is_empty()
    {
        let mut world : Vec<Organism> = Vec::with_capacity(3);
        for _i in 0..world.capacity()
        {
            world.push(Organism::new("a".to_owned()));
        }
        world[0].genome = "".to_owned();
        world[2].genome = "".to_owned();
        
        assert_eq!(find_closest_empty_index(&world, 1).unwrap(), 2);
    }
    #[test]
    fn test_find_closest_returns_next_index_downwards_if_upwards_cell_is_not_empty()
    {
        let mut world : Vec<Organism> = Vec::with_capacity(3);
        for _i in 0..world.capacity()
        {
            world.push(Organism::new("a".to_owned()));
        }
        world[0].genome = "".to_owned();
        
        
        assert_eq!(find_closest_empty_index(&world, 1).unwrap(), 2);
    }
}
