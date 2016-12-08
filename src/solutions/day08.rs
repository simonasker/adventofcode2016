extern crate regex;

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

struct Screen {
    array: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Self {
        Screen {
            array: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.array[y][x] = 1;
    }

    fn rect(&mut self, a: usize, b: usize) {
        for i in 0..b {
            for j in 0..a {
                self.turn_on(j, i);
            }
        }
    }

    fn rotate_column(&mut self, x: usize, n: u32) {
        for _ in 0..n {
            let last = self.array[SCREEN_HEIGHT-1][x];
            for i in (1..SCREEN_HEIGHT).rev() {
                self.array[i][x] = self.array[i-1][x];
            }
            self.array[0][x] = last;
        }
    }

    fn rotate_row(&mut self, y: usize, n: u32) {
        for _ in 0..n {
            let last = self.array[y][SCREEN_WIDTH-1];
            for i in (1..SCREEN_WIDTH).rev() {
                self.array[y][i] = self.array[y][i-1];
            }
            self.array[y][0] = last;
        }
    }

    fn count_pixels(&self) -> u32 {
        let mut total: u32 = 0;

        // TODO This could probably be done nicer
        for a in self.array.iter() {
            let sum: u8 = a.iter().sum();
            total += sum as u32;
        }

        total
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for a in self.array.iter() {
            for x in a.iter() {
                match x {
                    &0u8 => output.push('.'),
                    &1u8 => output.push('#'),
                    _ => { },
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}


fn part_one() {
    let f = File::open("input/day08.txt").unwrap();
    let reader = BufReader::new(f);

    let mut screen = Screen::new();

    let rect_re = regex::Regex::new(r"^rect (\d+)x(\d)$").unwrap();
    let rotate_column_re = regex::Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
    let rotate_row_re = regex::Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();

        if let Some(caps) = rect_re.captures(&line) {
            screen.rect(
                usize::from_str(caps.at(1).unwrap()).unwrap(),
                usize::from_str(caps.at(2).unwrap()).unwrap(),
            );
            continue;
        }

        if let Some(caps) = rotate_column_re.captures(&line) {
            screen.rotate_column(
                usize::from_str(caps.at(1).unwrap()).unwrap(),
                u32::from_str(caps.at(2).unwrap()).unwrap(),
            );
            continue;
        }

        if let Some(caps) = rotate_row_re.captures(&line) {
            screen.rotate_row(
                usize::from_str(caps.at(1).unwrap()).unwrap(),
                u32::from_str(caps.at(2).unwrap()).unwrap(),
            );
            continue;
        }
    }
    println!("{}", screen);
    println!("Sum: {}", screen.count_pixels());

}

fn part_two() {
    println!("Not yet implemented");
}
