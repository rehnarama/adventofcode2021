use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

const BOARD_SIZE: usize = 5;

#[derive(Clone, Copy)]
struct BingoNumber {
    number: i32,
    marked: bool,
}
impl fmt::Display for BingoNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number, if self.marked { "*" } else { "" })
    }
}
#[derive(Clone)]
struct BingoBoard {
    data: Vec<Vec<BingoNumber>>,
}
impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = (0..BOARD_SIZE).fold(String::new(), |board_str, y| {
            let line_str = (0..BOARD_SIZE).fold(String::new(), |line_str, x| {
                let piece = self.get(x, y).unwrap();
                line_str + &format!("{},", piece.to_string())
            });

            board_str + &format!("{}\n", line_str)
        });

        write!(f, "{}", str)
    }
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            data: vec![
                vec![
                    BingoNumber {
                        number: 0,
                        marked: false
                    };
                    BOARD_SIZE
                ];
                BOARD_SIZE
            ],
        }
    }

    fn from_string(str: &str) -> Result<BingoBoard, String> {
        // println!("From str: {}", str);
        let regex = match Regex::new(r"(\s|\n)+") {
            Ok(regex) => regex,
            Err(e) => return Err(e.to_string()),
        };
        let numbers = regex.split(str.trim());

        let mut board = BingoBoard::new();
        for (i, number_str) in numbers.enumerate() {
            let x = i % BOARD_SIZE;
            let y = ((i as f32) / BOARD_SIZE as f32) as usize;
            // println!("x={}, y={}, number_str={}", x, y, number_str);

            let number = match number_str.parse::<i32>() {
                Ok(number) => number,
                Err(e) => return Err(e.to_string()),
            };

            match board.set(x, y, number) {
                Err(_) => return Err("Failed to set number".to_string()),
                _ => (),
            }
        }

        Ok(board)
    }

    fn get(&self, x: usize, y: usize) -> Result<&BingoNumber, ()> {
        Ok(self.data.get(x).ok_or(())?.get(y).ok_or(())?)
    }

    fn set(&mut self, x: usize, y: usize, number: i32) -> Result<(), ()> {
        self.get_mut(x, y)?.number = number;
        Ok(())
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut BingoNumber, ()> {
        Ok(self.data.get_mut(x).ok_or(())?.get_mut(y).ok_or(())?)
    }

    fn mark_numbers(&mut self, number: i32) -> Result<(), ()> {
        for x in 0..5 {
            for y in 0..5 {
                let mut piece = self.get_mut(x, y)?;
                if piece.number == number {
                    piece.marked = true;
                }
            }
        }

        Ok(())
    }

    fn has_bingo(&self) -> Result<bool, ()> {
        for x in 0..5 {
            let mut n_marked = 0;
            for y in 0..5 {
                let piece = self.get(x, y)?;
                if piece.marked {
                    n_marked = n_marked + 1;
                }
            }
            if n_marked == 5 {
                return Ok(true);
            }
        }
        for y in 0..5 {
            let mut n_marked = 0;
            for x in 0..5 {
                let piece = self.get(x, y)?;
                if piece.marked {
                    n_marked = n_marked + 1;
                }
            }
            if n_marked == 5 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn get_unmarked(&self) -> Result<Vec<i32>, ()> {
        let mut unmarked = Vec::new();
        (0..BOARD_SIZE).for_each(|x| {
            (0..BOARD_SIZE).for_each(|y| {
                let piece = self.get(x, y).unwrap();
                if !piece.marked {
                    unmarked.push(piece.number);
                }
            });
        });

        Ok(unmarked)
    }
}

fn main() -> io::Result<()> {
    let file = fs::File::open("./input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let inputs = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = input
        .lines()
        .skip(2)
        .collect::<Vec<&str>>()
        .join("\n")
        .split("\n\n")
        .map(|board_str| BingoBoard::from_string(board_str).unwrap())
        .collect::<Vec<BingoBoard>>();

    for input in inputs {
        for board in boards.iter_mut() {
            board.mark_numbers(input).unwrap();
            if board.has_bingo().unwrap() {
                println!("BINGO!\n{}", board);
                let unmarked = board.get_unmarked().unwrap();
                let unmarked_sum = unmarked.iter().map(|n| *n).reduce(|a, b| a + b).unwrap();
                let product = unmarked_sum * input;
                println!("Unmarked sum={}, product={}", unmarked_sum, product);

                return Ok(());
            }
        }
    }

    Ok(())
}
