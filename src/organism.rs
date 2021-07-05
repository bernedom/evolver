#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Organism {
    pub genome: String,
    pub age: u32,
    pub max_age: u32,
}

impl Organism {
    pub fn new(g: String) -> Self {
        Self {
            genome: g,
            age: 0,
            max_age: 200,
        }
    }
    pub fn is_alive(&self) -> bool {
        return &self.genome != "";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let o = Organism::new(String::from("abc"));
        assert_eq!(0, o.age);
        assert_eq!("abc", o.genome);
        assert_eq!(200, o.max_age);
    }

    #[test]
    fn test_organism_with_empty_genome_is_dead() {
        let o = Organism::new("".to_string());
        assert_eq!(false, o.is_alive());
    }
    #[test]
    fn test_organism_with_non_empty_genome_is_alive() {
        let o = Organism::new("x".to_string());
        assert_eq!(true, o.is_alive());
    }
}
