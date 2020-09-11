#[macro_use]
extern crate lazy_static;

use rand::{thread_rng, Rng};

use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref ROBOT_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut r = Robot {
            name: String::new(),
        };
        r.reset_name();
        r
    }

    pub fn name(&self) -> &str {
        //println!("Name is {}", self.name);
        &self.name
    }

    pub fn reset_name(&mut self) {
        if !self.name.is_empty() {
            let mut active_names = ROBOT_NAMES.lock().unwrap();
            active_names.remove(&self.name);
        }

        loop {
            let mut robot_name = String::new();
            robot_name.push(thread_rng().gen_range(b'A', b'Z') as char);
            robot_name.push(thread_rng().gen_range(b'A', b'Z') as char);
            let number = rand::random::<u16>() % 1000;
            robot_name.push_str(format!("{:03}", number).as_str());
            let mut active_names = ROBOT_NAMES.lock().unwrap();
            if !active_names.contains(&robot_name) {
                active_names.insert(robot_name.clone());
                self.name = robot_name;
                break;
            }
        }
    }
}
