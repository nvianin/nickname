use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
/// Nickname is a random name generator
/// It appends a random number of trailing characters to a random name
pub struct Nickname {
    names: Vec<String>,
    trailing_chars: u8,
}

// Dataset from https://data.world/alexandra/baby-names
const NAMES: &str = include_str!("./names.txt");

impl Nickname {
    pub fn new(trailing_chars: u8) -> Nickname {
        let data = NAMES.replace("\r", "");
        let names: Vec<String> = data.split("\n").map(String::from).collect();

        Nickname {
            names,
            trailing_chars,
        }
    }

    pub fn name(&self) -> String {
        let i = thread_rng().gen_range(0..self.names.len());
        let mut name = self.names[i].clone();
        /* match self.names[i].chars().nth(0) {
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
        } */

        name += &(String::from("-") + &self.random_chars(self.trailing_chars));

        name
    }

    pub fn with_collision_avoidance(&self, names: &Vec<String>) -> String {
        let mut name = self.name();
        while names.contains(&name) {
            name = self.name();
        }
        name
    }

    fn random_chars(&self, number: u8) -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(number as usize)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::Nickname;
    use std::collections::HashMap;
    use std::thread;
    use std::time::Instant;

    pub const INSERTS_TO_TRY: usize = 100_000_000;
    #[test]
    pub fn median_nickname_collision_test() {
        let mut results = Vec::new();
        println!("Running 10 tests");
        let start = Instant::now();
        for _ in 0..10 {
            let start = Instant::now();
            results.push(nickname_test());
            println!("Test took {}ms", start.elapsed().as_millis());
        }
        println!("Total time: {}ms", start.elapsed().as_millis());
        results.sort();
        println!("Results: {:?}", results);
        println!("Total collisions: {}", results.iter().sum::<usize>());
        println!("Median: {}", results[5]);
        println!("Mean: {}", results.iter().sum::<usize>() / 10);
        println!("Max: {}", results[9]);
        println!("Min: {}", results[0]);
    }
    pub fn nickname_test() -> usize {
        let mut threads = Vec::new();
        let available_parallelism = thread::available_parallelism().unwrap().get();
        for _ in 0..available_parallelism {
            let thread: thread::JoinHandle<HashMap<String, u32>> = thread::spawn(move || {
                let namer = Nickname::new(5);
                let mut names: HashMap<String, u32> = HashMap::new();
                let mut collision_count = 0;
                let mut collisions: HashMap<String, u32> = HashMap::new();
                for n in 0..(INSERTS_TO_TRY / available_parallelism) {
                    if n % 100000 == 0 {
                        println!(
                            "{} inserts out of {}",
                            n,
                            INSERTS_TO_TRY / available_parallelism
                        );
                    }
                    let name = namer.name();
                    /* let surname = namer.name();
                    println!("{name} {surname}"); */
                    if let Some(_) = names.insert(name.clone(), 1) {
                        println!("Fail after {n} inserts, {} already exists", name);
                        let mut count = names.get(&name).unwrap().clone();
                        count += 1;
                        names.insert(name.clone(), count);
                        collisions.insert(name, count);
                        collision_count += 1;
                    }
                }
                println!(
                    "Thread {:?} finished with {} collisions",
                    thread::current().id(),
                    collision_count
                );
                collisions
            });
            threads.push(thread);
        }

        let mut names: HashMap<String, u32> = HashMap::new();
        let mut finished_threads = 0;
        for thread in threads {
            let results = thread.join().unwrap();
            names.extend(results);
            finished_threads += 1;
            println!(
                "{}/{} threads finished",
                finished_threads, available_parallelism
            );
        }

        println!("Parsing test results...");
        let mut collisions = HashMap::new();
        let mut i = 0;
        names.iter().for_each(|(k, v)| {
            if i % 100000 == 0 {
                println!("{}/{}...", i, INSERTS_TO_TRY);
            }
            i += 1;
            if *v > 1 {
                collisions.insert(k, v);
            }
        });

        println!("{:#?}", collisions);
        println!("Test finished with {} collisions", collisions.len());
        collisions.len()
    }
}
