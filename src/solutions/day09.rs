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
                let chars: String = char_iter
                    .by_ref()
                    .take(num_chars as usize)
                    .collect();
                println!("{:?}", chars);
                for _ in 0..reps {
                    output.push_str(&chars);
                }
            },
            Some(c) => {
                output.push(c);
            },
            None => break,
        }
    }

    println!("Input length: {}", input.len());
    println!("Output length: {}", output.len());

}

fn split_marker(marker_str: String) -> (u32, u32) {
    let mut split = marker_str.split('x');
    let num_chars = u32::from_str(split.next().unwrap()).unwrap();
    let reps = u32::from_str(split.next().unwrap()).unwrap();
    (num_chars, reps)
}

fn unwind(input: &str) -> u64 {
    println!("Unwinding: {}", input);
    let mut result = 0u64;
    let mut char_iter = input.chars();
    loop {
        let c = char_iter.next();
        match c {
            Some('(') => {
                let marker: String = char_iter
                    .by_ref()
                    .take_while(|&c| c != ')')
                    .collect();
                let (num_chars, reps) = split_marker(marker);
                let chars: String = char_iter
                    .by_ref()
                    .take(num_chars as usize)
                    .collect();

                let unwinded = unwind(chars.as_ref());
                result += reps as u64 * unwinded;
            },
            Some(_) => {
                result += 1;
            },
            None => break,
        }
    }

    println!("Returning: {}", result);
    result
}


fn part_two() {
    let mut f = File::open("input/day09.txt").unwrap();

    let mut input = String::new();

    f.read_to_string(&mut input).unwrap();

    let result = unwind(input.trim());

    println!("Result: {}", result);

}
