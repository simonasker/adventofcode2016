use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let f = File::open("input/day06.txt").unwrap();
    let reader = BufReader::new(f);

    let mut maps: Vec<HashMap<char, u32>> = Vec::new();

    for _ in 0..6 {
        maps.push(HashMap::new());
    }

    for line in reader.lines() {
        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            let ref mut letters = maps[i];
            let counter = letters.entry(c).or_insert(0);
            *counter += 1;
        }

    }

    for map in maps {
        println!("{:?}", map);
    }
}

fn part_two() {
    println!("Not yet implemented");
}
