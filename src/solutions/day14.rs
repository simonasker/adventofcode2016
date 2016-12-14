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
    let mut char_to_ind: HashMap<char, Vec<u32>> = HashMap::new();
    let mut ind_to_ctr: HashMap<u32, u32> = HashMap::new();

    'outer: for n in 0..201 {
        let mut hasher = Md5::new();
        let foo = format!("{}{}", INPUT, n);
        hasher.input_str(&foo);
        let hash = hasher.result_str();
        println!("{}: {}", foo, hash);

        let mut char_iter = hash.chars().peekable();

        let mut num = 0;
        let mut prev = '-';

        while let Some(c) = char_iter.next() {
            if c == prev { num += 1; } else { num = 1; }

            let mut next = '-';
            if let Some(cn) = char_iter.peek() {
                next = *cn;
            }

            if (num == 3 && c != next) || (num == 4 && c != next) {
                println!("FOUND TRIPLE");
                char_to_ind.entry(c).or_insert(Vec::new()).push(n);
                ind_to_ctr.insert(n, 1000);
                break;
            }

            if num == 5 {
                println!("FOUND QUINTUPLE");
                break;
            }


            prev = c.clone();
        }

        for val in ind_to_ctr.values_mut() {
            if *val > 0 {
                *val -= 1;
            }
        }
    }

    println!("{:?}", char_to_ind);
    println!("{:?}", ind_to_ctr);
}


fn part_two() {
    println!("Not yet implemented");
}
