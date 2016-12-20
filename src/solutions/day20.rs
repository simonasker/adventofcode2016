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
    let f = File::open("input/day20.txt").unwrap();
    let reader = BufReader::new(&f);

    let mut intervals: Vec<(u32, u32)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split('-');
        let low = split.next().unwrap().parse::<u32>().unwrap();
        let high = split.next().unwrap().parse::<u32>().unwrap();

        intervals.push((low, high));
    }

    let mut lowest_high = 0;

    loop {
        let mut done = true;
        for &(low, high) in &intervals {
            if low <= lowest_high+1 && high > lowest_high {
                lowest_high = high;
                println!("New lowest_high: {}", lowest_high);
                done = false;
            }
        }

        if done {
            break;
        }
    }

    println!("Answer: {}", lowest_high+1);
}

fn part_two() {
    println!("Not yet implemented");
}
