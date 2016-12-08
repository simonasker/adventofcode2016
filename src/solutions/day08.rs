use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const SCREEN_WIDTH: usize = 7;
const SCREEN_HEIGHT: usize = 3;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

#[derive(Debug)]
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


fn part_one() {
    let f = File::open("input/day08.txt").unwrap();
    let reader = BufReader::new(f);

    let mut screen = Screen::new();
    screen.rect(3, 2);
    println!("{:?}", screen);
    println!("Sum: {}", screen.count_pixels());

    for line in reader.lines() {
        let line = line.unwrap();

        println!("{}", line);

    }

}

fn part_two() {
    println!("Not yet implemented");
}
