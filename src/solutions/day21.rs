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

fn part_one() {
    let f = File::open("input/day21_example.txt").unwrap();
    let reader = BufReader::new(f);

    let re_1 = regex::Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    let re_2 = regex::Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        // Swap position X with position Y
        if let Some(caps) = re_1.captures(&line) {
            let x = i32::from_str(caps.at(1).unwrap()).unwrap();
            let y = i32::from_str(caps.at(2).unwrap()).unwrap();

            println!("-> SWAP POSITION: {} {}", x, y);
        }

        // Swap letter X with letter Y
        if let Some(caps) = re_2.captures(&line) {
            let x = caps.at(1).unwrap();
            let y = caps.at(2).unwrap();

            println!("-> SWAP LETTER: {} {}", x, y);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
