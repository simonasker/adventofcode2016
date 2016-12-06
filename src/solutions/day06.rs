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

    for _ in 0..8 {
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

    let mut answer = String::new();

    for map in maps {
        let mut chars: Vec<char> = map.keys().map(char::to_owned).collect();
        chars.sort_by_key(|c| map.get(c).unwrap());
        chars.reverse();
        answer.push(chars[0]);
    }

    println!("Answer: {}", answer);
}

fn part_two() {
    let f = File::open("input/day06.txt").unwrap();
    let reader = BufReader::new(f);

    let mut maps: Vec<HashMap<char, u32>> = Vec::new();

    for _ in 0..8 {
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

    let mut answer = String::new();

    for map in maps {
        let mut chars: Vec<char> = map.keys().map(char::to_owned).collect();
        chars.sort_by_key(|c| map.get(c).unwrap());
        // Removing this is the only part that is different from part_one
        // chars.reverse();
        answer.push(chars[0]);
    }

    println!("Answer: {}", answer);
}
