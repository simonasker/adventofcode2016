extern crate crypto;

use std::collections::HashMap;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

// const INPUT: &'static str = "zpeqevtbw";
const INPUT: &'static str = "abc";

fn part_one() {
    let mut potential: HashMap<char, Vec<u32>> = HashMap::new();
    let mut found_keys: u32 = 0;

    'outer: for n in 0.. {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", INPUT, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();

        // println!("{}: {}", foo, hash);

        let mut prev = '-';
        let mut num = 0;
        let mut char_iter = hash.chars().peekable();

        while let Some(c) = char_iter.next() {
            if c == prev {
                num += 1;
            } else {
                num = 1;
            }

            let mut next_char = '-';
            if let Some(nc) = char_iter.peek() {
                next_char = *nc;
            }

            if num == 3 && !(next_char == c) {
                println!("{}: Found triplet", n);
                let vec = potential.entry(c).or_insert(Vec::new());
                vec.push(1000);
                break;
            }

            if num == 5 {
                println!("{}: Found quintuplet", n);
                let vec = potential.get_mut(&c).expect("There must be a vector for this value");
                for val in vec.iter_mut() {
                    if *val > 0 {
                        println!("{}: Found key '{}'", n, c);
                        found_keys += 1;
                        if found_keys == 64 {
                            break 'outer;
                        }
                        *val = 0;
                    }
                }
                break;
            }
            prev = c.clone();
        }

        for vec in potential.values_mut() {
            for val in vec.iter_mut() {
                if *val > 0 {
                    *val -= 1;
                }
            }
        }
    }

    println!("Found keys: {}", found_keys);
    // println!("{:?}", potential);
}

fn part_two() {
    println!("Not yet implemented");
}
