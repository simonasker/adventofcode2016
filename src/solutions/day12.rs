use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

    let max: usize = instructions.len();
    let mut ptr: usize = 0;

    loop {
        let mut spl = instructions[ptr].split_whitespace();

        let inst = spl.next().unwrap();

        match inst {
            "cpy" => {
                println!("CPY");
            },
            "inc" => {
                println!("INC");
            },
            "dec" => {
                println!("DEC");
            },
            "jnz" => {
                println!("JNZ");
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


}

fn part_two() {
    println!("Not yet implemented");
}
