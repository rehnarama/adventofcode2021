use std::collections::HashMap;

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut mappings = HashMap::new();
    mappings.insert('(', ')');
    mappings.insert('[', ']');
    mappings.insert('{', '}');
    mappings.insert('<', '>');

    let mut stack = Vec::new();

    for char in line.chars() {
        let is_opening = mappings.contains_key(&char);
        if is_opening {
            stack.push(char);
        } else if let Some(prev_opening) = stack.pop() {
            let expected = match mappings.get(&prev_opening) {
                Some(expected) => expected,
                None => panic!("No matching mapping for {}", prev_opening),
            };

            if *expected != char {
                return Err(char);
            }
        }
    }

    Ok(stack)
}

fn get_completion(stack: Vec<char>) -> Vec<char> {
    let mut mappings = HashMap::new();
    mappings.insert('(', ')');
    mappings.insert('[', ']');
    mappings.insert('{', '}');
    mappings.insert('<', '>');

    stack
        .iter()
        .rev()
        .map(|opening_char| match mappings.get(opening_char) {
            Some(closing_char) => closing_char,
            None => panic!("No matching mapping for {}", opening_char),
        })
        .cloned()
        .collect()
}

fn get_completion_for_line(line: &str) -> Result<Vec<char>, char> {
    match parse_line(line) {
        Ok(stack) => Ok(get_completion(stack)),
        Err(corrupted_char) => Err(corrupted_char),
    }
}

fn get_corrupted_char(line: &str) -> Option<char> {
    match parse_line(line) {
        Ok(_) => None,
        Err(corrupted_char) => Some(corrupted_char),
    }
}

fn get_corrupted_points(corrupted_chars: &Vec<char>) -> i32 {
    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    corrupted_chars.iter().fold(0, |total, char| {
        let points = match points.get(char) {
            Some(points) => points,
            None => panic!("Couldn't find points for {}", char),
        };
        total + points
    })
}
fn get_completed_points(completed_chars: &Vec<char>) -> u64 {
    let mut points = HashMap::new();
    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);

    completed_chars.iter().fold(0, |total, char| {
        let points = match points.get(char) {
            Some(points) => points,
            None => panic!("Couldn't find points for {}", char),
        };
        total * 5 + points
    })
}

fn main() {
    let input = include_str!("../input.txt");

    let completions: Vec<Vec<char>> = input
        .lines()
        .map(|line| get_completion_for_line(line))
        .flatten()
        .collect();

    let mut points: Vec<u64> = completions
        .iter()
        .map(|completion| get_completed_points(completion))
        .collect();

    points.sort();

    let middle_index = points.len() / 2;
    let middle_point = match points.get(middle_index) {
        Some(middle_point) => middle_point,
        None => panic!("Couldn't get middle point at index {}", middle_index),
    };

    dbg!(middle_point);
}
