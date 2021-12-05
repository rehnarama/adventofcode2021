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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}
impl Line {
    fn from_string(str: String) -> Result<Line, String> {
        let (p1_str, p2_str) = match str.split_once("->") {
            Some(points) => points,
            None => return Err(format!("Couldn't parse string to line, str={}", str)),
        };

        let p1 = Point::from_string(p1_str.to_string())?;
        let p2 = Point::from_string(p2_str.to_string())?;

        Ok(Line { p1, p2 })
    }

    fn iter(&self) -> LineIterator {
        LineIterator::new(self.clone())
    }

    fn is_straight(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.p1, self.p2)
    }
}
struct LineIterator {
    line: Line,
    delta: Point,
    current: Point,
}
impl LineIterator {
    fn new(line: Line) -> LineIterator {
        LineIterator {
            delta: (line.p2 - line.p1).signum(),
            current: line.p1.clone(),
            line,
        }
    }
}
impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current == (self.line.p2 + self.delta) {
            return None;
        }

        let to_return = Some(self.current);
        self.current = self.current + self.delta;

        to_return
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

struct HydrothermalVentData {
    grid: Grid,
}
impl HydrothermalVentData {
    fn new(lines: &Vec<Line>) -> HydrothermalVentData {
        let width = lines.iter().fold(0, |current_max, line| {
            max(current_max, max(line.p1.x, line.p2.x) + 1)
        }) as usize;
        let height = lines.iter().fold(0, |current_max, line| {
            max(current_max, max(line.p1.y, line.p2.y) + 1)
        }) as usize;

        let mut grid = Grid::new(width, height);
        dbg!(width, height);

        for line in lines {
            for point in line.iter() {
                let current = match grid.get(point.x as usize, point.y as usize) {
                    Some(point) => point,
                    None => panic!("Couldn't get point {} in line {}", point, line),
                };
                let new = current + 1;
                grid.set(point.x as usize, point.y as usize, new).unwrap();
            }
        }

        HydrothermalVentData { grid }
    }

    fn number_of_overlapping_lines(&self) -> i32 {
        let mut n = 0;
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let value = self.grid.get(x, y).unwrap();
                if *value >= 2 {
                    n = n + 1
                }
            }
        }

        n
    }

    fn print_map(&self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let value = self.grid.get(x, y).unwrap();
                if *value == 0 {
                    print!("*");
                } else {
                    print!("{}", value);
                }
            }
            println!("");
        }
    }
}

fn main() -> io::Result<()> {
    let file = fs::File::open("./input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let lines: Vec<Line> = match input
        .lines()
        .map(|line_str| Line::from_string(line_str.to_string()))
        .collect()
    {
        Ok(lines) => lines,
        Err(e) => panic!("{}", e),
    };


    let data = HydrothermalVentData::new(&lines);
    data.print_map();

    println!(
        "Number of overlapping lines={}",
        data.number_of_overlapping_lines()
    );

    Ok(())
}

