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

    for n in 0..100 {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", INPUT, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();

        // print!("{}: ", foo);
        // println!("{}", hash);

        let mut prev = '-';
        let mut num = 0;
        for c in hash.chars() {
            if c == prev {
                num += 1;
            } else {
                num = 1;
            }

            if num == 3 {
                potential.insert(c, 1000);
            }
            prev = c.clone();
        }
    }

    println!("{:?}", potential);
}

fn part_two() {
    println!("Not yet implemented");
}
