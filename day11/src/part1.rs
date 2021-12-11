use ansi_term::Colour::{Blue, Red};
use std::cmp::max;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};
use std::{thread, time};

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
    data: Vec<Vec<(i32, bool)>>,
}
impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: vec![vec![(0, false); height]; width],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&(i32, bool)> {
        match self.data.get(x) {
            Some(column) => column.get(y),
            None => None,
        }
    }

    fn set(&mut self, x: usize, y: usize, number: (i32, bool)) -> Result<(), String> {
        match self.get_mut(x, y) {
            Some(cell) => {
                *cell = number;
                Ok(())
            }
            None => Err(format!("Couldn't find position ({}, {})", x, y)),
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut (i32, bool)> {
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
struct Octopuses {
    grid: Grid,
    n_flashes: usize,
}
impl Octopuses {
    fn new(grid: Grid) -> Octopuses {
        Octopuses { grid, n_flashes: 0 }
    }

    fn print_map(&self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let value = self.grid.get(x, y).unwrap();
                if value.0 == 0 {
                    print!("{}", Red.paint(value.0.to_string()));
                } else if value.0 > 9 {
                    print!("{}", Blue.paint("*"));
                } else {
                    print!("{}", value.0);
                }
            }
            println!("");
        }
    }

    fn get_surrounding_points(&self, point: Point) -> Vec<Point> {
        let mut surrounding_points = Vec::new();

        let Point { x, y } = point;

        for delta_x in -1..=1 {
            for delta_y in -1..=1 {
                let target_x = x as i32 + delta_x;
                let target_y = y as i32 + delta_y;
                if target_y < 0 || target_y >= self.grid.height() as i32 {
                    continue;
                }
                if target_x < 0 || target_x >= self.grid.width() as i32 {
                    continue;
                }
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                surrounding_points.push(Point {
                    x: target_x,
                    y: target_y,
                });
            }
        }

        surrounding_points
    }

    fn increment_by_one(&mut self) {
        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                let cell = self.grid.get_mut(x, y).unwrap();
                cell.0 = cell.0 + 1;
            }
        }
    }

    fn cells_with_nine(&self) -> Vec<Point> {
        let mut cells = Vec::new();

        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                if self.grid.get(x, y).unwrap().0 > 9 {
                    cells.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        cells
    }

    fn flash(&mut self, point: Point) -> Vec<Point> {
        let mut flashed_points = vec![point];

        let cell = self
            .grid
            .get_mut(point.x as usize, point.y as usize)
            .unwrap();

        if cell.1 {
            return flashed_points;
        } else {
            self.n_flashes += 1;
            cell.1 = true;
        }

        let surrounding_points = self.get_surrounding_points(point);
        for point in surrounding_points.iter() {
            let cell = self
                .grid
                .get_mut(point.x as usize, point.y as usize)
                .unwrap();
            cell.0 = cell.0 + 1;

            if cell.0 > 9 && !cell.1 {
                let mut new_flashed_points = self.flash(*point);
                flashed_points.append(&mut new_flashed_points);
            }
        }

        flashed_points
    }

    fn step(&mut self) {
        self.increment_by_one();

        let cells_with_nine = self.cells_with_nine();
        let mut flashed_points = Vec::new();
        for cell in cells_with_nine.iter() {
            let mut new_flashed_points = self.flash(*cell);
            flashed_points.append(&mut new_flashed_points);
        }

        for point in flashed_points {
            let cell = self
                .grid
                .get_mut(point.x as usize, point.y as usize)
                .unwrap();

            cell.0 = 0;
            cell.1 = false;
        }
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
        grid.set(x, y, (height, false)).unwrap();
    });

    let mut octopuses = Octopuses::new(grid);
    println!("------------ Initial state ----------------");
    octopuses.print_map();
    for step in 1..=100 {
        octopuses.step();
        println!("------------ After step {} ----------------", step);
        octopuses.print_map();
    }

    println!("n flashes={}", octopuses.n_flashes);
}
