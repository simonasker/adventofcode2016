use std::fs::File;
use std::io::prelude::*;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}


fn part_one() {
    let mut f = File::open("input/day18.txt").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    input = input.trim().to_owned();


    let mut map: Vec<String> = Vec::new();

    let size = input.len();

    map.push(input.clone());

    for _ in 0..9 {
        let mut chars: Vec<char> = input.chars().collect();
        chars.insert(0, '.');
        chars.push('.');

        let mut new_row = String::new();

        for i in 1..size+1 {
            if chars[i-1] == chars[i] && chars[i] != chars[i+1] {
                new_row.push('^');
            } else if chars[i+1] == chars[i] && chars[i] != chars[i-1] {
                new_row.push('^');
            } else {
                new_row.push('.');
            }
        }

        map.push(new_row.clone());
        input = new_row;

    }


    for row in map {
        println!("{}", row);
    }

}

fn part_two() {
    println!("Not yet implemented");
}
