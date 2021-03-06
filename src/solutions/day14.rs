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

const INPUT: &'static str = "zpqevtbw";
// const INPUT: &'static str = "abc";

fn part_one() {
    let mut char_to_ind: HashMap<char, Vec<u32>> = HashMap::new();
    let mut ind_to_ctr: HashMap<u32, u32> = HashMap::new();

    let mut found_keys = 0;

    'outer: for n in 0.. {
        for val in ind_to_ctr.values_mut() {
            if *val > 0 {
                *val -= 1;
            }
        }

        let mut hash_input = format!("{}{}", INPUT, n);

        // TODO Only do the hashing once for solution to part 1
        for _ in 0..2017 {
            let mut hasher = Md5::new();
            hasher.input_str(&hash_input);
            hash_input = hasher.result_str();
        }

        let mut char_iter = hash_input.chars();

        let mut num = 0;
        let mut prev = '-';

        let mut found_three = false;

        while let Some(c) = char_iter.next() {
            if c == prev { num += 1; } else { num = 1; }


            if !found_three && num == 3 {
                // println!("TRIPLE: {} {}", foo, hash);
                char_to_ind.entry(c).or_insert(Vec::new()).push(n);
                ind_to_ctr.insert(n, 1001);

                found_three = true;
            }

            if num == 5 {
                if let Some(indices) = char_to_ind.get(&c) {
                    for i in indices {
                        let ctr = ind_to_ctr.get_mut(&i).unwrap();
                        if *ctr > 0 && *ctr <= 1000 {
                            found_keys += 1;
                            println!("FOUND KEY: {} ({})", found_keys, i);
                            // Reset the counter to avoid counting a key twice
                            *ctr = 0;

                            if found_keys == 64 {
                                break 'outer;
                            }
                        }
                    }
                }
            }

            prev = c.clone();
        }

    }

    println!("Found keys: {}", found_keys);
}


fn part_two() {
    println!("Not yet implemented");
}
