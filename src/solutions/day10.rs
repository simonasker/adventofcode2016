extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

#[derive(Debug)]
struct Bot {
    values: Vec<u32>,
}

impl Bot {
    fn new() -> Self {
        Bot {
            values: Vec::new(),
        }
    }

    fn give(&mut self, value: u32) {
        if self.values.len() < 2 {
            self.values.push(value);
        } else {
            println!("BOT FULL");
        }
    }

    fn take_low(&mut self) -> Option<u32> {
        self.values.sort();
        self.values.reverse();
        self.values.pop()
    }

    fn take_high(&mut self) -> Option<u32> {
        self.values.sort();
        self.values.pop()
    }
}


fn part_one() {
    let f = File::open("input/day10.txt").unwrap();
    let reader = BufReader::new(f);

    let input_re = regex::Regex::new(
        r"^value (\d+) goes to bot (\d+)$").unwrap();
    let give_re = regex::Regex::new(
        r"^bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)$").unwrap();

     let mut bots: Vec<Bot> = Vec::new();

     let num_bots = 3;
     for _ in 0..num_bots {
         bots.push(Bot::new());
     }

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut done = false;
    while !done {

        let mut line_iter = lines.clone().into_iter();
        for line in line_iter.by_ref() {
            println!("{}", line);
            if let Some(caps) = input_re.captures(&line) {
                let value = u32::from_str(caps.at(1).unwrap()).unwrap();
                let bot = usize::from_str(caps.at(2).unwrap()).unwrap();

                bots[bot].give(value);
            }
        }
    }


    // let f = File::open("input/day10.txt").unwrap();
    // let reader = BufReader::new(f);
    // for line in reader.lines() {
    //     let line = line.unwrap();

    //     if let Some(caps) = give_re.captures(&line) {
    //         let bot = usize::from_str(caps.at(1).unwrap()).unwrap();

    //         let low_val = bots[bot].take_low().unwrap();
    //         let high_val = bots[bot].take_high().unwrap();

    //         let low_type = caps.at(2).unwrap();
    //         let low = usize::from_str(caps.at(3).unwrap()).unwrap();
    //         if low_type == "bot" {
    //             bots[low].give(low_val);
    //         }

    //         let high_type = caps.at(4).unwrap();
    //         let high = usize::from_str(caps.at(5).unwrap()).unwrap();
    //         if high_type == "bot" {
    //             bots[high].give(high_val);
    //         }
    //     }
    // }
}

fn part_two() {
    println!("Not yet implemented");
}
