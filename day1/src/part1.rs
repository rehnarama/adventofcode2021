use std::fs;
use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::File::open("./input.txt")?;

    let reader = io::BufReader::new(input);
    let (_, increments) = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .fold((i32::max_value(), 0), |(prev_value, increments), value| {
            (value, increments + if value > prev_value { 1 } else { 0 })
        });

    println!("Number of increments: {}", increments);

    Ok(())
}
