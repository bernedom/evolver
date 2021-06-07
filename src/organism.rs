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
            max_age: 100,
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
        assert_eq!(100, o.max_age);
    }
}
