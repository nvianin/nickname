use std::time::Instant;


/// Nickname is a random name generator
// Dataset from https://data.world/alexandra/baby-names
const NAMES: &str  = include_str!("./names.txt");

pub fn generate() -> String {
    let names: Vec<&str> = NAMES.split("\n").collect();
    let name = names[rand::random::<usize>() % names.len()];
    name.to_string()
}

pub fn test() {
    let then = Instant::now();

    let to_generate = 100000;
    for _ in 0..to_generate {
        generate();
    }

    println!("Generated {} names in {:?}", to_generate, then.elapsed());
}