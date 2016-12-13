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
    let f = File::open("input/day12.txt").unwrap();
    let reader = BufReader::new(f);

    let instructions: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not get line"))
        .collect();

    let max = instructions.len() as i32;
    let mut ptr = 0;

    let mut register: HashMap<char, i32> = HashMap::new();
    register.insert('a', 0);
    register.insert('b', 0);
    // Set this to 1 for solution to part 2
    register.insert('c', 0);
    register.insert('d', 0);

    loop {
        let mut spl = instructions[ptr as usize].split_whitespace();

        let inst = spl
            .next().expect("There must be at least one word on each line");

        match inst {
            "cpy" => {
                let arg1 = spl
                    .next().expect("cpy has a first argument");

                let reg = spl
                    .next().expect("cpy has a second argument")
                    .chars().nth(0).expect("The second argument is at least one character");

                // println!("CPY {} {}", arg1, reg);

                let mut new_val = 0;

                if let Ok(v) = arg1.parse::<i32>() {
                    new_val = v;
                } else if let Some(other_reg) = arg1.chars().nth(0) {
                    new_val = *register.get(&other_reg).expect("There should be a value here");
                }

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val = new_val;
                ptr += 1;
            },
            "inc" => {
                let reg = spl
                    .next().expect("inc has one arguement")
                    .chars().nth(0).expect("The argument to inc has length one");

                // println!("INC {}", reg);

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val += 1;
                ptr += 1;
            },
            "dec" => {
                let reg = spl
                    .next().expect("dec has one argument")
                    .chars().nth(0).expect("The argument to dec has length one");

                // println!("DEC {}", reg);

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val -= 1;
                ptr += 1;
            },
            "jnz" => {
                let arg1 = spl
                    .next().expect("jnz has a first argument");
                let val = spl
                    .next().expect("jnz has a second argument")
                    .parse::<i32>().expect("The second argument to jnz is na int");

                // println!("JNZ {} {}", reg, val);
                let mut new_val = 0;

                if let Ok(v) = arg1.parse::<i32>() {
                    new_val = v;
                } else if let Some(other_reg) = arg1.chars().nth(0) {
                    new_val = *register.get(&other_reg).expect("There should be a value here");
                }

                if new_val != 0 {
                    ptr += val;
                } else {
                    ptr += 1;
                }
            },
            _ => {
                panic!("Invalid instruction");
            },
        }
        print!("\r{:?}", register);


        if ptr >= max {
            break;
        }
    }

    println!("\n{:?}", register);
}

fn part_two() {
    println!("Not yet implemented");
}
