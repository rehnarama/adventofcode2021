use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Steering {
    x: i32,
    y: i32,
    aim: i32,
}

impl Add for Steering {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            aim: self.aim + other.aim,
        }
    }
}

impl fmt::Display for Steering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.aim)
    }
}

fn main() -> io::Result<()> {
    let input = fs::File::open("./input.txt")?;

    let reader = io::BufReader::new(input);
    let destination = reader.lines().map(|line| line.unwrap()).fold(
        Steering { x: 0, y: 0, aim: 0 },
        |steering, command| {
            let (command, value) = command.split_once(" ").unwrap();
            let value = value.parse::<i32>().unwrap();

            let delta = match command {
                "forward" => Steering {
                    x: value,
                    y: steering.aim * value,
                    aim: 0,
                },
                "up" => Steering {
                    x: 0,
                    y: 0,
                    aim: -value,
                },
                "down" => Steering {
                    x: 0,
                    y: 0,
                    aim: value,
                },
                _ => {
                    println!("Invalid command found! {}", command);
                    Steering { x: 0, y: 0, aim: 0 }
                }
            };

            steering + delta
        },
    );

    let product = destination.x * destination.y;

    println!("Destination product: {}", product);

    Ok(())
}
