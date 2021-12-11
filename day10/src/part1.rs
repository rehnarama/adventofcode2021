use std::collections::HashMap;

fn get_corrupted_char(line: &str) -> Option<char> {
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
                return Some(char);
            }
        }
    }

    None
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

fn main() {
    let input = include_str!("../input.txt");

    let corrupted_chars: Vec<char> = input
        .lines()
        .map(|line| get_corrupted_char(line))
        .flatten()
        .collect();

    let points = get_corrupted_points(&corrupted_chars);

    dbg!(points);
}
