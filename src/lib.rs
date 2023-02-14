use rand::{thread_rng, Rng};
use std::fs;

pub struct NameGen {
    names: Vec<String>,
}

impl NameGen {
    pub fn new() -> NameGen {
        let data = fs::read_to_string("./names.txt").unwrap().replace("\r", "");
        let names: Vec<String> = data.split("\n").map(String::from).collect();

        NameGen { names }
    }

    pub fn name(&mut self) -> String {
        let i = thread_rng().gen_range(0..self.names.len());
        let mut name: String;
        if self.names[i].chars().nth(0).unwrap() != '/' {
            name = self.names[i].clone();
        } else {
            name = self.name();
        }

        name += &(String::from("-") + &self.random_chars(3));

        name
    }

    fn random_chars(&mut self, number: usize) -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(number)
            .map(char::from)
            .collect()
    }
}

#[test]
pub fn test() {
    let mut namer = NameGen::new();
    for _ in 0..100 {
        println!("{}", namer.name());
    }
}
