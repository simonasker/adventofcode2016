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

    let mut intervals: Vec<(u64, u64)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split('-');
        let low = split.next().unwrap().parse::<u64>().unwrap();
        let high = split.next().unwrap().parse::<u64>().unwrap();

        intervals.push((low, high));
    }

    let mut lowest_high = 0;

    let mut allowed_ips = 0;

    loop {
        if lowest_high >= 4294967295 {
            break;
        }

        let mut done = true;
        for &(low, high) in &intervals {
            if low <= lowest_high+1 && high > lowest_high {
                lowest_high = high;
                println!("New lowest_high: {}", lowest_high);
                done = false;
            }
        }

        if done {
            allowed_ips += 1;
            println!("New IP: {}", lowest_high+1);
            lowest_high += 1;
        }
    }

    println!("Answer: {}", allowed_ips);
}

fn part_two() {
    println!("Not yet implemented");
}
