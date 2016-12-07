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
        self.queue[0] == self.queue[3] &&
            self.queue[1] == self.queue[2] &&
            self.queue[0] != self.queue[1]
    }
}

#[derive(Debug)]
struct AbaQueue {
    queue: [char; 3],
}

impl AbaQueue {
    fn new() -> Self {
        AbaQueue {
            queue: ['-'; 3],
        }
    }

    fn add(&mut self, c: char) {
        self.queue[0] = self.queue[1];
        self.queue[1] = self.queue[2];
        self.queue[2] = c;
    }

    fn is_aba(&self) -> bool {
        self.queue[0] == self.queue[2] &&
            self.queue[0] != self.queue[1]
    }
}

fn part_one() {
    let f = File::open("input/day07.txt").unwrap();
    let reader = BufReader::new(f);

    let mut tls_ips = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut chars = line.chars();
        let mut inside = false;
        let mut aq = AbbaQueue::new();
        let mut abba_inside = false;
        let mut abba_outside = false;

        loop {
            if let Some(c) = chars.next() {
                match c {
                    '[' => {
                        inside = true;
                        aq = AbbaQueue::new();
                    },
                    ']' => {
                        inside = false;
                        aq = AbbaQueue::new();
                    },
                    _ => {
                        aq.add(c);
                        if aq.is_abba() {
                            if inside {
                                abba_inside = true;
                            } else {
                                abba_outside = true;
                            }
                        }
                    },
                }
            } else {
                break;
            }
        }

        if abba_outside && !abba_inside {
            tls_ips += 1;
        }
    }

    println!("Answer: {}", tls_ips);
}

fn part_two() {
    let f = File::open("input/day07_test.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut chars = line.chars();
        let mut inside = false;
        let mut aq = AbaQueue::new();

        println!("{}", line);
        loop {
            if let Some(c) = chars.next() {
                match c {
                    '[' => {
                        inside = true;
                        aq = AbaQueue::new();
                    },
                    ']' => {
                        inside = false;
                        aq = AbaQueue::new();
                    },
                    _ => {
                        aq.add(c);
                        if aq.is_aba() {
                            println!("{:?}", aq);
                            // if inside {
                            //     abba_inside = true;
                            // } else {
                            //     abba_outside = true;
                            // }
                        }
                    },
                }
            } else {
                break;
            }
        }

        // if abba_outside && !abba_inside {
        //     tls_ips += 1;
        // }
    }

    // println!("Answer: {}", tls_ips);
}
