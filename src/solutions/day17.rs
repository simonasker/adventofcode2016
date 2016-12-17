extern crate crypto;

// use std::collections::HashMap;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

// const INPUT: &'static str = "ihgpwlah"; // DDRRRD
// const INPUT: &'static str = "kglvqrro"; // DDUDRLRRUDRD
// const INPUT: &'static str = "ulqzkmiv"; // DRURDRUDDLLDLUURRDULRLDUUDDDRR

const INPUT: &'static str = "vwbaicqe";

fn find_room(path: &str) -> Option<(i32, i32)> {
    let mut room_x = 0;
    let mut room_y = 0;

    for d in path.chars() {
        match d {
            'U' => room_y -= 1,
            'D' => room_y += 1,
            'L' => room_x -= 1,
            'R' => room_x += 1,
            _ => panic!("Invalid direction"),
        }
    }

    if room_x < 0 || room_x > 3 {
        return None;
    }

    if room_y < 0 || room_y > 3 {
        return None;
    }

    Some((room_x, room_y))
}

fn open_char(c: char) -> bool {
    !(c == 'a' || c.is_digit(10))
}

fn door_is_open(path: &str, dir: &str) -> bool {
    let hash_input = format!("{}{}", INPUT, path);

    let mut hasher = Md5::new();
    hasher.input_str(&hash_input);
    let hash = hasher.result_str();

    match dir {
        "U" => return open_char(hash.chars().nth(0).unwrap()),
        "D" => return open_char(hash.chars().nth(1).unwrap()),
        "L" => return open_char(hash.chars().nth(2).unwrap()),
        "R" => return open_char(hash.chars().nth(3).unwrap()),
        _ => panic!("Invalid direction"),
    }
}


fn neighbors(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for dir in vec!["U", "D", "L", "R"] {
        let new_path = format!("{}{}", path, dir);

        if let Some(_) = find_room(&new_path) {
            if door_is_open(path, dir) {
                result.push(new_path);
            }
        }
    }
    result
}

fn part_one() {
    let start = String::from("");

    let mut q: Vec<String> = Vec::new();
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();

        println!("{}", current);

        if find_room(&current) == Some((3, 3)) {
            println!("Valid path: {}", current);
            break;
        }

        for n in neighbors(&current) {
            q.insert(0, n);
        }
    }

}

fn part_two() {
    println!("{:?}", find_room("RRDDD"));
    println!("{:?}", find_room("DDRRRD"));

    println!("{:?}", door_is_open("", "U"));
    println!("{:?}", door_is_open("", "D"));
    println!("{:?}", door_is_open("", "L"));
    println!("{:?}", door_is_open("", "R"));
}
