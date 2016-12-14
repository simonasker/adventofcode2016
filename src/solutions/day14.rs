extern crate crypto;

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
    for n in 0..20 {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", INPUT, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();

        print!("{}: ", foo);
        println!("{}", hash);

        let mut prev = '-';
        let mut num = 0;
        for c in hash.chars() {
            if c == prev {
                num += 1;
            } else {
                num = 1;
            }

            if num == 3 {
                println!("FOUND A TRIPLET");
            }
            prev = c.clone();
        }
        print!("\n");
    }
}

fn part_two() {
    println!("Not yet implemented");
}
