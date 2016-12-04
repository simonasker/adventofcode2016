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

fn part_one() {
    let f = File::open("input/day02.txt").unwrap();
    let reader = BufReader::new(f);

    let (mut x, mut y) = (1, 2);

    for line in reader.lines() {
        let line = line.unwrap();

        for c in line.chars() {
            match c {
                'R' => if x < 2 { x += 1; },
                'L' => if x > 0 { x -= 1; },
                'U' => if y > 0 { y -= 1; },
                'D' => if y < 2 { y += 1; },
                _ => println!("Invalid direction"),
            }
        }

        let button = 3 * y + x + 1;
        print!("{}", button);
    }

    println!("");
}

fn part_two() {
    let f = File::open("input/day02.txt").unwrap();
    let reader = BufReader::new(f);

    let buttons = [
        '0', '0', '1', '0', '0',
        '0', '2', '3', '4', '0',
        '5', '6', '7', '8', '9',
        '0', 'A', 'B', 'C', '0',
        '0', '0', 'D', '0', '0',
    ];

    let (mut x, mut y) = (0, 2);

    for line in reader.lines() {
        let line = line.unwrap();

        for c in line.chars() {

            let (min_x, max_x) = match y {
                0 | 4 => (2, 2),
                1 | 3 => (1, 3),
                2     => (0, 4),
                _     => panic!(),
            };

            let (min_y, max_y) = match x {
                0 | 4 => (2, 2),
                1 | 3 => (1, 3),
                2     => (0, 4),
                _     => panic!(),
            };

            match c {
                'R' => if x < max_x { x += 1; },
                'L' => if x > min_x { x -= 1; },
                'U' => if y > min_y { y -= 1; },
                'D' => if y < max_y { y += 1; },
                _ => println!("Invalid direction"),
            }
        }

        let button_index = 5 * y + x;
        print!("{}", buttons[button_index]);
    }
    println!("");
}
