use rand::Rng;
use std::{cell::RefCell, collections::HashSet};

#[derive(Debug)]
pub struct Robot {
    name: String,
}

thread_local!(static ROBOTS: RefCell<HashSet<String>>  = RefCell::new(HashSet::new()));

impl Robot {
    pub fn new() -> Self {
        let roboto = Robot {
            name: Robot::make_unique_name(),
        };

        println!("New robot: {:?}", roboto);

        roboto
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::make_unique_name();
    }

    pub fn generate_name() -> String {
        let mut rng = rand::thread_rng();

        let mut name = String::new();

        name.push(rng.gen_range(b'A'..b'Z') as char);
        name.push(rng.gen_range(b'A'..b'Z') as char);
        name.push(rng.gen_range(b'0'..b'9') as char);
        name.push(rng.gen_range(b'0'..b'9') as char);
        name.push(rng.gen_range(b'0'..b'9') as char);
        name
    }

    pub fn make_unique_name() -> String {
        ROBOTS.with(|c| {
            let mut robots = c.borrow_mut();

            loop {
                let try_name = Robot::generate_name();

                if !robots.contains(&try_name) {
                    robots.insert(try_name.clone());

                    break try_name;
                }
            }
        })
    }
}
