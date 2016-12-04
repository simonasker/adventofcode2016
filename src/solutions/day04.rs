use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let f = File::open("input/day04.txt").unwrap();
    let reader = BufReader::new(f);

    let mut sector_id_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut split: Vec<&str> = line
            .split_terminator(|c| !char::is_alphanumeric(c))
            .collect();

        let checksum = split.pop().unwrap();
        let sector_id = split.pop().unwrap().parse::<i32>().unwrap();


        let mut letters = HashMap::new();

        for c in line.chars() {
            if c == '-' {
                continue;
            } else if char::is_numeric(c) {
                break;
            } else {
                let counter = letters.entry(c).or_insert(0);
                *counter += 1;
            }
        }

        // TODO can `map(|c| *c)` be done in a nicer way?
        let mut chars: Vec<char> = letters.keys().map(|c| *c).collect();

        chars.sort_by(|a, b| {
            let a_num = *letters.get(a).unwrap();
            let b_num = *letters.get(b).unwrap();

            match a_num.cmp(&b_num) {
                Ordering::Equal => {
                    return b.cmp(a);
                },
                ord @ _ => return ord,
            }
        });

        chars.reverse();

        let actual_checksum: String = chars.into_iter().take(5).collect();

        if checksum == actual_checksum {
            sector_id_sum += sector_id;
        }
    }

    println!("Sector ID sum: {}", sector_id_sum);
}

fn part_two() {
    let f = File::open("input/day04.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut split: Vec<&str> = line
            .split_terminator(|c| !char::is_alphanumeric(c))
            .collect();

        let _ = split.pop().unwrap();
        let sector_id = split.pop().unwrap().parse::<u32>().unwrap();

        let split: Vec<char> = split.concat().chars().map(|c| {
            ((((c as u32 - 96) + sector_id) % 26 + 96) as u8) as char
        }).collect();

        let string: String = split.into_iter().collect();

        if string.contains("northpole") {
            println!("{}", sector_id);
            break;
        }
    }
}
