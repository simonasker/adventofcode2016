extern crate regex;

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

    let mut bot = Bot::new();
    bot.give(10);
    bot.give(12);
    println!("{:?}", bot);
    println!("{:?}", bot.take_low());
    bot.give(18);
    println!("{:?}", bot.take_high());
    println!("===================================================");

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        if let Some(caps) = input_re.captures(&line) {
            let value = caps.at(1).unwrap();
            let bot = caps.at(2).unwrap();
            println!("Value: {}, Bot: {}", value, bot);
        }

        if let Some(caps) = give_re.captures(&line) {
            let bot = caps.at(1).unwrap();
            let low_type = caps.at(2).unwrap();
            let low = caps.at(3).unwrap();

            let high_type = caps.at(4).unwrap();
            let high = caps.at(5).unwrap();

            println!("{}", caps.at(1).unwrap());
            println!("Bot: {}, H -> {} {}, L -> {} {}", bot, low_type, low, high_type, high);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
