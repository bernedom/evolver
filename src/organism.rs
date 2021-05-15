#[derive(Default)]
pub struct Organism {
    pub genome: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let o: Organism = Default::default();
        assert_eq!(0, o.age);
        assert_eq!("", o.genome);
    }
}
