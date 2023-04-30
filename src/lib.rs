use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NameGen {
    names: Vec<String>,
}

// Dataset from https://data.world/alexandra/baby-names
const NAMES: &str = include_str!("./names.txt");

impl NameGen {
    pub fn new() -> NameGen {
        let data = NAMES.replace("\r", "");
        let names: Vec<String> = data.split("\n").map(String::from).collect();

        NameGen { names }
    }

    pub fn name(&self) -> String {
        let i = thread_rng().gen_range(0..self.names.len());
        let mut name: String;
        match self.names[i].chars().nth(0) {
            Some(char) => {
                if char != '/' {
                    name = self.names[i].clone();
                } else {
                    name = self.name();
                }
            }
            None => {
                name = self.name();
            }
        }

        name += &(String::from("-") + &self.random_chars(5));

        name
    }

    fn random_chars(&self, number: usize) -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(number)
            .map(char::from)
            .collect()
    }
}

#[test]
pub fn test() {
    let namer = NameGen::new();
    let mut names = HashMap::new();
    let mut collision_count = 0;
    for n in 0..1_000_000 {
        if n % 100000 == 0 {
            println!("{} inserts", n);
        }
        let name = namer.name();
        /* let surname = namer.name();
        println!("{name} {surname}"); */
        if let Some(_) = names.insert(name.clone(), 1) {
            println!("Fail after {n} inserts, {} already exists", name);
            names.insert(name.clone(), names.get(&name).unwrap() + 1);
            collision_count += 1;
        }
    }
    let mut collisions = HashMap::new();

    names.iter().for_each(|(k, v)| {
        if *v > 1 {
            collisions.insert(k, v);
        }
    });

    println!("{:#?}", collisions);
    println!("Test finished with {} collisions", collision_count);
}
