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

    let start = "abcde";

    let mut chars: Vec<char> = start.chars().collect();

    println!("{:?}\n", chars);


    let re_1 = regex::Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    let re_2 = regex::Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap();
    let re_3 = regex::Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    let re_4 = regex::Regex::new(r"^rotate (\w+) (\d+) step.*$").unwrap();
    let re_5 = regex::Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
    let re_6 = regex::Regex::new(r"^rotate based on position of letter (\w)$").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        // Swap position X with position Y
        if let Some(caps) = re_1.captures(&line) {
            let x = usize::from_str(caps.at(1).unwrap()).unwrap();
            let y = usize::from_str(caps.at(2).unwrap()).unwrap();
            let temp = chars[x];
            chars[x] = chars[y];
            chars[y] = temp;
            println!("-> {:?}", chars);
        }

        // Swap letter X with letter Y
        if let Some(caps) = re_2.captures(&line) {
            let x = caps.at(1).unwrap().chars().nth(0).unwrap();
            let y = caps.at(2).unwrap().chars().nth(0).unwrap();
            let ix = chars.iter().position(|&c| c == x).unwrap();
            let iy = chars.iter().position(|&c| c == y).unwrap();
            let temp = chars[ix];
            chars[ix] = chars[iy];
            chars[iy] = temp;
            println!("-> {:?}", chars);
        }

        // Reverse positions X through position Y
        if let Some(caps) = re_3.captures(&line) {
            let x = usize::from_str(caps.at(1).unwrap()).unwrap();
            let y = usize::from_str(caps.at(2).unwrap()).unwrap();
            let mut sub = Vec::new();
            for i in x..y+1 {
                sub.push(chars[i]);
            }
            sub.reverse();
            let mut sub_iter = sub.iter();
            for i in x..y+1 {
                chars[i] = *sub_iter.next().unwrap();
            }
            println!("-> {:?}", chars);
        }

        // Rotate DIR X steps
        if let Some(caps) = re_4.captures(&line) {
            let dir = caps.at(1).unwrap();
            let x = i32::from_str(caps.at(2).unwrap()).unwrap();
            match dir {
                "left" => {
                    for _ in 0..x {
                        let c = chars.remove(0);
                        chars.push(c);
                    }
                },
                "right" => {
                    for _ in 0..x {
                        let c = chars.pop().unwrap();
                        chars.insert(0, c);
                    }
                },
                _ => panic!("Invalid direction"),
            }
            println!("-> {:?}", chars);
        }

        // Move position X to position Y
        if let Some(caps) = re_5.captures(&line) {
            let x = usize::from_str(caps.at(1).unwrap()).unwrap();
            let y = usize::from_str(caps.at(2).unwrap()).unwrap();
            let c = chars.remove(x);
            chars.insert(y, c);
            println!("-> {:?}", chars);
        }

        // Rotate based on position of letter X
        if let Some(caps) = re_6.captures(&line) {
            let x = caps.at(1).unwrap().chars().nth(0).unwrap();
            let ix = chars.iter().position(|&c| c == x).unwrap();
            let mut steps = 1 + ix;
            if ix >= 4 { steps += 1; }
            for _ in 0..steps {
                let c = chars.pop().unwrap();
                chars.insert(0, c);
            }
            println!("-> {:?}", chars);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
