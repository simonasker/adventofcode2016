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

#[derive(Debug)]
struct AbbaQueue {
    queue: [char; 4],
}

impl AbbaQueue {
    fn new() -> Self {
        AbbaQueue {
            queue: ['-'; 4],
        }
    }

    fn add(&mut self, c: char) {
        self.queue[0] = self.queue[1];
        self.queue[1] = self.queue[2];
        self.queue[2] = self.queue[3];
        self.queue[3] = c;
    }

    fn is_abba(&self) -> bool {
        self.queue[0] == self.queue[3] && self.queue[1] == self.queue[2]
    }
}

fn part_one() {
    let f = File::open("input/day07.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        let mut chars = line.chars();
        let mut inside = false;
        loop {
            if let Some(c) = chars.next() {
                match c {
                    '[' => inside = true,
                    ']' => inside = false,
                    _ => {
                        if inside {
                            println!("IN  : {}", c);
                        } else {
                            println!("OUT : {}", c);
                        }
                    },
                }
            } else {
                break;
            }
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
