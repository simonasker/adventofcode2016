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

fn part_one() {
    let f = File::open("input/day09.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn part_two() {
    println!("Not yet implemented");
}
