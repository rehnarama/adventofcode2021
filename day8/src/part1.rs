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

    let inputs: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(signals, output)| {
            let parsed: (Vec<&str>, Vec<&str>) = (
                signals.trim().split(" ").collect(),
                output.trim().split(" ").collect(),
            );
            return parsed;
        })
        .collect();

    let output_lengths: Vec<usize> = inputs
        .iter()
        .map(|(_signals, outputs)| outputs)
        .flat_map(|outputs| outputs.iter().map(|output| output.len()))
        .collect();

    let outputs_with_2_3_4_or_7_digits: Vec<usize> = output_lengths
        .iter()
        .cloned()
        .filter(|length| {
            let length = *length as i32;
            length == 2 || length == 3 || length == 4 || length == 7
        })
        .collect();

    let how_many = outputs_with_2_3_4_or_7_digits.len();

    println!("{} outputs with unique segments", how_many);

    Ok(())
}
