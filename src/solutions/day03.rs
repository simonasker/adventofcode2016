use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn is_valid(t: &Vec<i32>) -> bool {
    t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0]
}

fn part_one() {
    let f = File::open("input/day03.txt").unwrap();
    let reader = BufReader::new(f);

    let mut valid_triangles = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut split= line.split_whitespace();

        let a = split.next().unwrap().parse::<i32>().unwrap();
        let b = split.next().unwrap().parse::<i32>().unwrap();
        let c = split.next().unwrap().parse::<i32>().unwrap();

        if a + b > c && a + c > b && b + c > a {
            valid_triangles += 1;
        }
    }

    println!("Valid triangles: {}", valid_triangles);
}

fn part_two() {
    let f = File::open("input/day03.txt").unwrap();
    let reader = BufReader::new(f);

    let mut valid_triangles = 0;

    let lines = reader.lines();

    let mut counter = 0;

    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut split= line.split_whitespace();

        let a = split.next().unwrap().parse::<i32>().unwrap();
        let b = split.next().unwrap().parse::<i32>().unwrap();
        let c = split.next().unwrap().parse::<i32>().unwrap();

        if counter == 0 {
            t1 = vec![a];
            t2 = vec![b];
            t3 = vec![c];
            counter += 1;
        } else if counter == 1 {
            t1.push(a);
            t2.push(b);
            t3.push(c);
            counter += 1;
        } else if counter == 2 {
            t1.push(a);
            t2.push(b);
            t3.push(c);

            if is_valid(&t1) { valid_triangles += 1; }
            if is_valid(&t2) { valid_triangles += 1; }
            if is_valid(&t3) { valid_triangles += 1; }

            counter = 0;
        }
    }

    println!("Valid triangles: {}", valid_triangles);
}
