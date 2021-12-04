use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
struct AddableVec(Vec<i32>);

impl Add for AddableVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let inner_self = &self.0;
        let inner_other = &other.0;

        let longest = if inner_self.len() > inner_other.len() {
            inner_self
        } else {
            inner_other
        };
        let shortest = if longest == inner_self {
            inner_other
        } else {
            inner_self
        };

        let mut new: Vec<i32> = Vec::new();
        for (i, num) in longest.iter().enumerate() {
            let other_num = shortest.get(i).or(Some(&0)).unwrap();
            new.push(*num + *other_num);
        }

        AddableVec(new)
    }
}

fn main() -> io::Result<()> {
    let input = fs::File::open("./input.txt")?;

    let reader = io::BufReader::new(input);
    let numbers = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|bit| if bit.eq(&'1') { 1 } else { 0 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let length = numbers.get(0).unwrap().len();

    let mut oxygen = numbers.iter().collect::<Vec<&Vec<i32>>>();

    for pos in 0..length {
        let freqs = oxygen
            .iter()
            .fold(AddableVec(Vec::new()), |total_freqs, number| {
                let line_freqs = number
                    .iter()
                    .map(|bit| if *bit == 1 { 1 } else { -1 })
                    .collect::<Vec<i32>>();

                return total_freqs + AddableVec(line_freqs);
            });

        let freq = freqs.0.get(pos).unwrap();

        let most_common = (((freq.signum() + 1) as f64) / 2.0).ceil() as i32;
        oxygen = oxygen
            .iter()
            .filter(|number| *(number.get(pos).unwrap()) == most_common)
            .map(|number| *number)
            .collect::<Vec<&Vec<i32>>>();

        if oxygen.len() == 1 {
            break;
        }
    }

    let oxygen_rating = oxygen
        .get(0)
        .unwrap()
        .iter()
        .rev() // big-endian to little-endian for easier conversion
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate | (bit << pos));

    println!("Oxygen rating: {}", oxygen_rating);

    let mut co2 = numbers.iter().collect::<Vec<&Vec<i32>>>();

    for pos in 0..length {
        let freqs = co2
            .iter()
            .fold(AddableVec(Vec::new()), |total_freqs, number| {
                let line_freqs = number
                    .iter()
                    .map(|bit| if *bit == 1 { 1 } else { -1 })
                    .collect::<Vec<i32>>();

                return total_freqs + AddableVec(line_freqs);
            });

        let freq = freqs.0.get(pos).unwrap();

        let most_common = (((freq.signum() + 1) as f64) / 2.0).ceil() as i32;
        co2 = co2
            .iter()
            .filter(|number| *(number.get(pos).unwrap()) != most_common)
            .map(|number| *number)
            .collect::<Vec<&Vec<i32>>>();

        if co2.len() == 1 {
            break;
        }
    }

    let co2_rating = co2
        .get(0)
        .unwrap()
        .iter()
        .rev() // big-endian to little-endian for easier conversion
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate | (bit << pos));

    println!("CO2 rating: {}", co2_rating);

    println!("Product={}", oxygen_rating * co2_rating);

    Ok(())
}
