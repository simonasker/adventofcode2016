use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let mut f = File::open("input/day01.txt").unwrap();

    let mut file_contents = String::new();

    f.read_to_string(&mut file_contents).unwrap();

    println!("This is the file contents: {}", file_contents);

    let split = file_contents.trim().split(", ");

    let mut heading = 0;
    let mut v: i32 = 0;
    let mut h: i32 = 0;

    // let mut previous_locations: Vec<(i32, i32)> = Vec::new();
    let mut previous = HashSet::new();

    'outer: for s in split {
        let dir = match s.chars().nth(0) {
            Some('R') => 1,
            Some('L') => -1,
            _ => 0,
        };

        let dist = s[1..].parse::<i32>().unwrap();

        heading = (heading + dir) % 4;

        if heading < 0 {
            heading = 4 + heading;
        }

        for _ in 0..dist {
            match heading {
                0 => v += 1,
                1 => h += 1,
                2 => v -= 1,
                3 => h -= 1,
                _ => {},
            }
            println!("s: {}, dir: {}, heading: {}, dist: {}", s, dir, heading, dist);

            println!("v: {}, h: {}", v, h);

            if previous.contains(&(v,h)) {
                println!("FOUND LOCATION");
                break 'outer;
            }

            previous.insert((v, h));
        }

    }

    println!("V: {}, H: {}", v, h);
    let total_distance = v.abs() + h.abs();
    println!("Total distance: {}", total_distance);
}

fn part_two() {
    println!("Running part 2");
}
