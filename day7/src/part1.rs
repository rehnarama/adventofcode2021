use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

fn main() -> io::Result<()> {
    let file = fs::File::open("./input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let numbers: Vec<i32> = match input
        .trim()
        .split(",")
        .map(|str| str.parse::<i32>())
        .collect()
    {
        Ok(numbers) => numbers,
        Err(e) => panic!("Couldn't parse numbers: {}", e),
    };

    let highest_number = *numbers.iter().reduce(|a, b| a.max(b)).unwrap();

    let fuel_costs: Vec<i32> = (0..=highest_number)
        .map(|target| {
            numbers
                .iter()
                .fold(0, |fuel, number| fuel + (number - target).abs())
        })
        .collect();

    let smallest = fuel_costs
        .iter()
        .enumerate()
        .reduce(|a, b| if a.1 < b.1 { a } else { b })
        .unwrap();

    println!("Smallest cost at {} with cost {}", smallest.0, smallest.1);

    Ok(())
}
