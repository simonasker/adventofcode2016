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

    loop {
        let mut spl = instructions[ptr as usize].split_whitespace();

        let inst = spl
            .next().expect("There must be at least one word on each line");

        match inst {
            "cpy" => {
                let val = spl
                    .next().expect("cpy has a first argument")
                    .parse::<i32>().expect("The first argument of cpy is an int");
                let reg = spl
                    .next().expect("cpy has a second argument")
                    .chars().nth(0).expect("The second argument is at least one character");

                println!("CPY {} {}", val, reg);

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val = val;

            },
            "inc" => {
                let reg = spl
                    .next().expect("inc has one arguement")
                    .chars().nth(0).expect("The argument to inc has length one");

                println!("INC {}", reg);

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val += 1;
            },
            "dec" => {
                let reg = spl
                    .next().expect("dec has one argument")
                    .chars().nth(0).expect("The argument to dec has length one");

                println!("DEC {}", reg);

                let reg_val = register.entry(reg).or_insert(0);
                *reg_val -= 1;
            },
            "jnz" => {
                let reg = spl
                    .next().expect("jnz has a first argument")
                    .chars().nth(0).expect("The first argument to jnz has length one");
                let val = spl
                    .next().expect("jnz has a second argument")
                    .parse::<i32>().expect("The second argument to jnz is na int");

                println!("JNZ {} {}", reg, val);

                let reg_val = register.get(&reg).expect("There should be a value here");
                if *reg_val != 0 {
                    ptr += val;
                }
            },
            _ => {
                panic!("Invalid instruction");
            },
        }

        ptr += 1;

        if ptr >= max {
            break;
        }
    }

    println!("{:?}", register);
}

fn part_two() {
    println!("Not yet implemented");
}
