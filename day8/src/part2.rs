use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

fn main() -> io::Result<()> {
    let file = fs::File::open("./input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut output_value_sum = 0;

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

    let all_chars: HashSet<char> = collection! {'a', 'b', 'c', 'd', 'e', 'f', 'g'};
    for (signals, outputs) in inputs.iter() {
        let mut possible_a: HashSet<char> = collection! {'a', 'b', 'c', 'd', 'e', 'f', 'g'};
        let mut possible_b: HashSet<char> = possible_a.clone();
        let mut possible_c: HashSet<char> = possible_a.clone();
        let mut possible_d: HashSet<char> = possible_a.clone();
        let mut possible_e: HashSet<char> = possible_a.clone();
        let mut possible_f: HashSet<char> = possible_a.clone();
        let mut possible_g: HashSet<char> = possible_a.clone();

        for signal in signals.iter().chain(outputs.iter()) {
            let length = signal.len();
            match length {
                2 => {
                    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
                    for char in all_chars.difference(&chars) {
                        possible_c.remove(char);
                        possible_f.remove(char);
                    }
                    for char in chars.iter() {
                        possible_a.remove(char);
                        possible_b.remove(char);
                        possible_d.remove(char);
                        possible_e.remove(char);
                        possible_g.remove(char);
                    }
                }
                3 => {
                    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
                    for char in all_chars.difference(&chars) {
                        possible_a.remove(char);
                        possible_c.remove(char);
                        possible_f.remove(char);
                    }
                    for char in chars.iter() {
                        possible_b.remove(char);
                        possible_d.remove(char);
                        possible_e.remove(char);
                        possible_g.remove(char);
                    }
                }
                4 => {
                    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
                    for char in all_chars.difference(&chars) {
                        possible_b.remove(char);
                        possible_c.remove(char);
                        possible_d.remove(char);
                        possible_f.remove(char);
                    }
                    for char in chars.iter() {
                        possible_a.remove(char);
                        possible_e.remove(char);
                        possible_g.remove(char);
                    }
                }
                _ => {}
            }
        }

        let mut length_segment_map: Vec<Vec<&str>> = vec![vec![]; 8];

        for signal in signals.iter() {
            let length = signal.len();
            length_segment_map.get_mut(length).unwrap().push(signal);
        }

        let length_5: Vec<Vec<char>> = length_segment_map
            .get(5)
            .unwrap()
            .iter()
            .map(|signal| signal.chars().collect())
            .collect();

        let possible_letters: Vec<char> = length_5
            .iter()
            .flat_map(|signal| signal.iter().copied())
            .collect();

        let counts: Vec<(char, usize)> = possible_letters
            .iter()
            .map(|char| {
                let count = possible_letters.iter().filter(|c| *c == char).count();
                (*char, count)
            })
            .collect();

        let unique_at_5: Vec<char> = counts
            .iter()
            .filter(|(_char, count)| *count == 1)
            .map(|(char, _)| *char)
            .collect();

        let first_unique = unique_at_5.get(0).unwrap();
        let second_unique = unique_at_5.get(1).unwrap();
        let b = if possible_b.contains(first_unique) {
            *first_unique
        } else {
            *second_unique
        };
        let e = if possible_e.contains(first_unique) {
            *first_unique
        } else {
            *second_unique
        };

        possible_b = collection! { b };
        possible_e = collection! { e };

        possible_a.remove(&b);
        possible_a.remove(&e);
        possible_b.remove(&e);
        possible_c.remove(&b);
        possible_c.remove(&e);
        possible_d.remove(&b);
        possible_d.remove(&e);
        possible_e.remove(&b);
        possible_f.remove(&b);
        possible_f.remove(&e);
        possible_g.remove(&b);
        possible_g.remove(&e);

        let number_five: Vec<char> = length_5
            .iter()
            .filter(|chars| chars.contains(&b))
            .flat_map(|chars| chars.iter())
            .cloned()
            .collect();

        for char in number_five.iter() {
            possible_c.remove(char);
            possible_e.remove(char);
        }
        for char in possible_c.iter() {
            possible_f.remove(char);
        }

        let mut mappings = HashMap::new();
        let a = *possible_a
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let b = *possible_b
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let c = *possible_c
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let d = *possible_d
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let e = *possible_e
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let f = *possible_f
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        let g = *possible_g
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        mappings.insert(a, 'a');
        mappings.insert(b, 'b');
        mappings.insert(c, 'c');
        mappings.insert(d, 'd');
        mappings.insert(e, 'e');
        mappings.insert(f, 'f');
        mappings.insert(g, 'g');

        fn string_to_digit(str: &String) -> char {
            let zero: HashSet<_> = collection! { 'a', 'b', 'c', 'e', 'f', 'g' };
            let one: HashSet<_> = collection! {'c', 'f'};
            let two: HashSet<_> = collection! {'a', 'c', 'd', 'e', 'g'};
            let three: HashSet<_> = collection! {'a', 'c', 'd', 'f', 'g'};
            let four: HashSet<_> = collection! {'b', 'c', 'd', 'f'};
            let five: HashSet<_> = collection! {'a', 'b', 'd', 'f', 'g'};
            let six: HashSet<_> = collection! {'a', 'b', 'd', 'e', 'f', 'g'};
            let seven: HashSet<_> = collection! {'a', 'c', 'f'};
            let eight: HashSet<_> = collection! {'a', 'b', 'c', 'd', 'e', 'f', 'g'};
            let nine: HashSet<_> = collection! {'a', 'b', 'c', 'd', 'f', 'g'};
            let digits = vec![
                (zero, '0'),
                (one, '1'),
                (two, '2'),
                (three, '3'),
                (four, '4'),
                (five, '5'),
                (six, '6'),
                (seven, '7'),
                (eight, '8'),
                (nine, '9'),
            ];

            for (digit, number) in digits.iter() {
                let is_digit = str.chars().all(|char| digit.contains(&char));
                if digit.len() == str.len() && is_digit {
                    return *number;
                }
            }
            panic!("Couldn't find it");
        }

        let decoded_outputs: Vec<String> = outputs
            .iter()
            .map(|output| {
                output
                    .chars()
                    .map(|char| mappings.get(&char).unwrap())
                    .cloned()
                    .collect()
            })
            .collect();

        let number_str: String = decoded_outputs
            .iter()
            .map(|output| string_to_digit(output))
            .collect();

        let number = number_str.parse::<i32>().unwrap();
        output_value_sum = output_value_sum + number;
    }
    dbg!(output_value_sum);

    Ok(())
}
