fn main() {
    let world = vec![vec![0; 50]; 50];

    for line in world {
        for field in line {
            println!("{}", field);
        }
        println!("End\n");
    }
}
