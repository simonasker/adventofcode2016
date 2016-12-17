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


fn neighbors(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for dir in vec!["U", "D", "L", "R"] {
        let new_path = format!("{}{}", path, dir);

        if let Some(_) = find_room(&new_path) {
            result.push(new_path);
        }
    }
    result
}

fn part_one() {
    // let input = "ihgpwlah";
    let input = "hijkl";

    // let mut distances: HashMap<Node, u32> = HashMap::new();
    // let mut parents: HashMap<Node, Node> = HashMap::new();


    let path = String::new();
    let hash_input = format!("{}{}", input, path);
    let mut hasher = Md5::new();
    hasher.input_str(&hash_input);
    let mut hash = hasher.result_str();


    hash.truncate(4);

    println!("{}", hash);

    // let start = (1, 1);

    let start = String::from("");

    let mut q: Vec<String> = Vec::new();
    // distances.insert(start, 0);
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();
        println!("{}", current);
        if find_room(&current) == Some((4, 4)) {
            println!("Valid path: {}", current);
        }

        for n in neighbors(&current) {
            q.insert(0, n);
        }
    }

    println!("{:?}", find_room("RRDDD"));
    println!("{:?}", find_room("RRDDDD"));
}

fn part_two() {
    println!("Not yet implemented");
}
