use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let input = fs::File::open("./input.txt")?;

    let reader = io::BufReader::new(input);
    let (_, increments) = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
        .windows(3)
        .fold((i32::max_value(), 0), |(prev_sum, increments), window| {
            let window_sum = window.iter().map(|n| *n).reduce(|a, b| a + b).unwrap();
            (
                window_sum,
                increments + if window_sum > prev_sum { 1 } else { 0 },
            )
        });

    println!("Number of increments: {}", increments);

    Ok(())
}
