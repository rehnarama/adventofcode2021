use std::fs;
use std::io;
use std::io::prelude::*;

const MAX_DAYS: usize = 6;
const SPAWN_AGE: usize = 8;

struct School {
    fishes: Vec<u64>,
}
impl School {
    fn from_string(str: String) -> Result<School, String> {
        let mut fishes = vec![0; SPAWN_AGE + 1];

        let ages: Vec<i32> = match str
            .trim()
            .split(",")
            .map(|age_str| age_str.parse::<i32>())
            .collect()
        {
            Ok(ages) => ages,
            Err(e) => return Err(format!("Couldn't parse school due to {}", e)),
        };

        ages.iter().for_each(|age| {
            let index = *age as usize;
            fishes[index] = fishes[index] + 1;
        });

        Ok(School { fishes })
    }

    fn tick(&mut self) {
        let n_spawners = self.fishes[0];
        for i in 0..SPAWN_AGE {
            self.fishes[i as usize] = self.fishes[i as usize + 1];
        }
        self.fishes[MAX_DAYS] = self.fishes[MAX_DAYS] + n_spawners;
        self.fishes[SPAWN_AGE] = n_spawners;
    }

    fn len(&self) -> usize {
        self.fishes
            .iter()
            .fold(0, |count, n_of_age| count + *n_of_age as usize)
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
    for i in 1..=256 {
        school.tick();
        println!("After day {}, n fishes={}", i, school.len());
    }

    Ok(())
}
