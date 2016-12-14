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
    let mut potential: HashMap<char, u32> = HashMap::new();
    let mut found_keys: u32 = 0;

    for n in 0..22730 {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", INPUT, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();


        let mut prev = '-';
        let mut num = 0;
        for c in hash.chars() {
            if c == prev {
                num += 1;
            } else {
                num = 1;
            }

            if num == 3 {
                let mut add = false;
                match potential.get(&c) {
                    None => {
                        add = true;
                    },
                    Some(cnt) => {
                        if *cnt == 0 {
                            add = true;
                        }
                    },
                }

                if add {
                    potential.insert(c, 1000);
                }
            }

            if num == 5 {
                if let Some(cnt) = potential.get(&c) {
                    if *cnt > 0 {
                        print!("{}: ", foo);
                        println!("{}", hash);
                        found_keys += 1;
                    }
                }
            }
            prev = c.clone();
        }

        for val in potential.values_mut() {
            if *val > 0 {
                *val -= 1;
            }
        }
    }

    println!("Found keys: {}", found_keys);
}

fn part_two() {
    println!("Not yet implemented");
}
