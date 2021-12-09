use ansi_term::Colour::Red;
use std::cmp::max;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn signum(&self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn from_string(str: String) -> Result<Point, String> {
        let (x_str, y_str) = match str.trim().split_once(",") {
            Some(s) => s,
            None => return Err(format!("Couldn't parse to point, str='{}'", str)),
        };

        let x = match x_str.parse::<i32>() {
            Ok(x) => x,
            Err(e) => return Err(format!("{}", e)),
        };
        let y = match y_str.parse::<i32>() {
            Ok(y) => y,
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(Point { x, y })
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Grid {
    data: Vec<Vec<i32>>,
}
impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: vec![vec![0; height]; width],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&i32> {
        match self.data.get(x) {
            Some(column) => column.get(y),
            None => None,
        }
    }

    fn set(&mut self, x: usize, y: usize, number: i32) -> Result<(), String> {
        match self.get_mut(x, y) {
            Some(cell) => {
                *cell = number;
                Ok(())
            }
            None => Err(format!("Couldn't find position ({}, {})", x, y)),
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut i32> {
        match self.data.get_mut(x) {
            Some(column) => column.get_mut(y),
            None => None,
        }
    }

    fn width(&self) -> usize {
        self.data.len()
    }
    fn height(&self) -> usize {
        self.data.get(0).unwrap().len()
    }
}
struct HeightMap {
    grid: Grid,
}
impl HeightMap {
    fn new(grid: Grid) -> HeightMap {
        HeightMap { grid }
    }

    fn print_map(&self) {
        let low_points = self.get_low_points();
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let value = self.grid.get(x, y).unwrap();
                if low_points.contains(&(x, y, *value)) {
                    print!("{}", Red.paint(value.to_string()));
                } else {
                    print!("{}", value);
                }
            }
            println!("");
        }
    }

    fn get_surrounding_points(&self, x: usize, y: usize) -> Vec<&i32> {
        let mut surrounding_points = Vec::new();

        for delta_x in vec![-1 as i32, 1] {
            let target_x = x as i32 + delta_x;
            if target_x < 0 || target_x >= self.grid.width() as i32 {
                continue;
            }
            let value = self.grid.get(target_x as usize, y).unwrap();
            surrounding_points.push(value);
        }
        for delta_y in vec![-1 as i32, 1] {
            let target_y = y as i32 + delta_y;
            if target_y < 0 || target_y >= self.grid.height() as i32 {
                continue;
            }

            let value = self.grid.get(x, target_y as usize).unwrap();
            surrounding_points.push(value);
        }

        surrounding_points
    }

    fn get_low_points(&self) -> Vec<(usize, usize, i32)> {
        let mut low_points = Vec::new();
        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                let surrounding_points = self.get_surrounding_points(x, y);

                let value = self.grid.get(x, y).unwrap();
                let is_low_point = surrounding_points
                    .iter()
                    .fold(true, |is_lower, surrounding_value| {
                        is_lower && (value < surrounding_value)
                    });

                if is_low_point {
                    low_points.push((x, y, *value));
                }
            }
        }

        low_points
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let data: Vec<(usize, usize, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .enumerate()
                .map(move |(x, height)| (x, y, height))
        })
        .collect();

    let (width, height): (usize, usize) = data
        .iter()
        .copied()
        .fold((0, 0), |(biggest_x, biggest_y), (x, y, _)| {
            (biggest_x.max(x), biggest_y.max(y))
        });

    let mut grid = Grid::new(width + 1, height + 1);
    data.iter().copied().for_each(|(x, y, height)| {
        grid.set(x, y, height).unwrap();
    });

    let height_map = HeightMap::new(grid);
    height_map.print_map();
    let low_points = height_map.get_low_points();
    let risk_sum = low_points
        .iter()
        .map(|(_, _, value)| value + 1)
        .fold(0, |total_risk, risk| total_risk + risk);

    dbg!(risk_sum);
}
