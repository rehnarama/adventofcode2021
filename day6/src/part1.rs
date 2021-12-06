use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

const MAX_DAYS: i32 = 6;
const SPAWN_AGE: i32 = 8;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Fish {
    age: i32,
}
impl Fish {
    fn tick(&mut self) -> bool {
        if self.age == 0 {
            self.age = MAX_DAYS;
            true
        } else {
            self.age = self.age - 1;
            false
        }
    }
}
struct School {
    fishes: Vec<Fish>,
}
impl School {
    fn from_string(str: String) -> Result<School, String> {
        let ages: Vec<i32> = match str
            .trim()
            .split(",")
            .map(|age_str| age_str.parse::<i32>())
            .collect()
        {
            Ok(ages) => ages,
            Err(e) => return Err(format!("Couldn't parse school due to {}", e)),
        };

        let fishes: Vec<Fish> = ages.iter().cloned().map(|age| Fish { age }).collect();

        Ok(School { fishes })
    }

    fn tick(&mut self) {
        let mut new_fishes = Vec::new();
        for fish in self.fishes.iter_mut() {
            if fish.tick() {
                new_fishes.push(Fish { age: SPAWN_AGE });
            }
        }

        self.fishes.append(&mut new_fishes);
    }

    fn len(&self) -> usize {
        self.fishes.len()
    }
}

fn main() -> io::Result<()> {
    let file = fs::File::open("./input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut school = match School::from_string(input) {
        Ok(school) => school,
        Err(e) => panic!("{}", e),
    };
    for i in 1..=80 {
        school.tick();
        println!("After day {} fishes={}", i, school.len());
    }

    Ok(())
}
