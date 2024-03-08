use std::time::Instant;

/// Nickname is a random name generator
// Dataset from https://data.world/alexandra/baby-names
const NAMES: &str = include_str!("./names.txt");

pub fn generate() -> String {
    // We get some chars
    let start = rand::random::<usize>() % (NAMES.len() - 30);
    let sampled = &NAMES[start..start + 30].chars();
    // Try to find a newline, then take the rest of the string up to another newline
    let mut name = String::new();
    let start = sampled.clone().position(|c| c == '\n').unwrap_or(0) + 1;
    
    for i in start..sampled.clone().count() {
        let c = sampled.clone().nth(i).unwrap();
        if c == '\n' {
            /* println!("Broke at {i}"); */
            break;
        }
        /* println!("Adding {c}"); */
        name.push(c);
    }
    name
}

pub fn test() {
    let then = Instant::now();

    let to_generate = 100000;
    for _ in 0..to_generate {
        println!("{}", generate());
    }

    println!("Generated {} names in {:?}", to_generate, then.elapsed());
}
