extern crate crypto;

use std::io::prelude::*;
use std::fs::File;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let mut f = File::open("input/day05.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let input = input.trim().to_owned();

    let mut hashes_found = 0;

    let mut answer = String::new();

    for n in 0.. {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", input, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();
        if hash.starts_with("00000") {
            println!("{}: {}", foo, hash);
            hashes_found += 1;
            answer.push(hash.chars().nth(5).unwrap());
            if hashes_found == 8 {
                break;
            }
        }

    }

    println!("Answer: {}", answer);
}

fn part_two() {
    let mut f = File::open("input/day05.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let input = input.trim().to_owned();

    let mut answer = ['-'; 8];
    let mut found = [false; 8];

    for n in 0.. {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", input, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();

        if hash.starts_with("00000") {
            let position = hash.chars().nth(5).unwrap();
            let character = hash.chars().nth(6).unwrap();

            if let Some(pos) = position.to_digit(10) {
                if pos < 8 && !found[pos as usize] {
                    println!("{}: {}", foo, hash);
                    answer[pos as usize] = character;
                    found[pos as usize] = true;
                    println!("{:?}", answer);
                }
            }
        }

        let mut done = true;

        for b in found.iter() {
            if !b {
                done = false;
            }
        }

        if done {
            break;
        }
    }

    println!("Answer: {:?}", answer);
}
