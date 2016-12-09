use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let mut f = File::open("input/day09.txt").unwrap();

    let mut input = String::new();
    let mut output = String::new();

    f.read_to_string(&mut input).unwrap();

    let mut char_iter = input.trim().chars();

    loop {
        let c = char_iter.next();
        match c {
            Some('(') => {
                let marker: String = char_iter
                    .by_ref()
                    .take_while(|&c| c != ')')
                    .collect();
                println!("{:?}", marker);
                let mut asd = marker.split('x');
                let num_chars = u32::from_str(asd.next().unwrap()).unwrap();
                let reps = u32::from_str(asd.next().unwrap()).unwrap();
                println!("Num chars: {}", num_chars);
                println!("Repetitions: {}", reps);
            },
            Some(c) => {
                output.push(c);
            },
            None => break,
        }
    }

    println!("{}", output);

}

fn part_two() {
    println!("Not yet implemented");
}
