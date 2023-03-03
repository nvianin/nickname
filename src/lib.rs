use core::panic;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NameGen {
    names: Vec<String>,
}

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

        name += &(String::from("-") + &self.random_chars(3));

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

use std::collections::HashMap;

#[test]
pub fn test() {
    let namer = NameGen::new();
    let mut names = HashMap::new();
    for n in 0..100_000_000 {
        let name = namer.name();
        println!("{}", name);
        if let Some(_) = names.insert(name.clone(), 1) {
            println!("Fail after {n} inserts, {} already exists", name);
            names.insert(name.clone(), names.get(&name).unwrap() + 1);
        }
    }
}
