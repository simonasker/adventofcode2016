extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

struct Node {
    used: u32,
    avail: u32,
}

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let f = File::open("input/day22.txt").unwrap();
    let reader = BufReader::new(f);

    let re = regex::Regex::new(
        r"^/dev/grid/node-x(\d+)-y(\d+)\s+\d+T\s+(\d+)T\s+(\d+)T\s+\d+%$")
        .expect("Invalid regex");

    let mut nodes: HashMap<(u32, u32), Node> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if let Some(caps) = re.captures(&line) {
            let x = u32::from_str(caps.at(1).unwrap()).unwrap();
            let y = u32::from_str(caps.at(2).unwrap()).unwrap();
            let used = u32::from_str(caps.at(3).unwrap()).unwrap();
            let avail = u32::from_str(caps.at(4).unwrap()).unwrap();

            nodes.insert((x, y), Node { used: used, avail: avail });
        }
    }

    let mut viable_pairs = 0;
    for a in nodes.keys() {
        for b in nodes.keys() {
            if a == b {
                continue;
            }

            let node_a = nodes.get(a).unwrap();
            let node_b = nodes.get(b).unwrap();

            if node_a.used > 0 && node_a.used <= node_b.avail {
                viable_pairs += 1;
            }
        }
    }

    // Answer to part 1
    println!("Viable pairs: {:?}", viable_pairs);

    // Print map to manually solve part 2
    for y in 0..28 {
        for x in 0..37 {
            let node = nodes.get(&(x, y)).unwrap();
            if x == 36 && y == 0 {
                print!("G");
            } else if node.used == 0 {
                print!("_");
            } else if node.used > 100 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part_two() {
    println!("Not yet implemented");
}
