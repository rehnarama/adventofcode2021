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
    let freqs = reader.lines().map(|line| line.unwrap()).fold(
        AddableVec(Vec::new()),
        |total_freqs, line| {
            let line_freqs = line
                .chars()
                .map(|bit| if bit.eq(&'1') { 1 } else { -1 })
                .collect::<Vec<i32>>();

            return total_freqs + AddableVec(line_freqs);
        },
    );

    let gamma_rate_binary = freqs.0.iter().map(|freq| (freq.signum() + 1) / 2);

    let gamma_rate = gamma_rate_binary
        .clone()
        .rev() // big-endian to little-endian for easier conversion
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate | (bit << pos));

    let epsilon_rate = gamma_rate_binary
        .clone()
        .map(|bit| if bit == 0 { 1 } else { 0 })
        .rev() // big-endian to little-endian for easier conversion
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate | (bit << pos));

    println!(
        "gamma_rate={}, epsilon_rate={}, product={}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    Ok(())
}
