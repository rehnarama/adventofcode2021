use std::ops::Add;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() -> io::Result<()> {
    let input = fs::File::open("./input.txt")?;

    let reader = io::BufReader::new(input);
    let destination = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(Point {x: 0, y: 0}, |point, command| {
            let (command, value) = command.split_once(" ").unwrap();
            let value = value.parse::<i32>().unwrap();

            let delta = match command {
                "forward" => Point {x: value, y: 0},
                "up" => Point{ x: 0, y: -value },
                "down" => Point{ x: 0, y: value },
                _ => {
                    println!("Invalid command found! {}", command);
                    Point {x: 0, y: 0}
                }
            };

            point + delta
        });

    let product = destination.x * destination.y;

    println!("Destination product: {}", product);

    Ok(())
}
