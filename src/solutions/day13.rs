use std::collections::HashMap;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

type Node = (u32, u32);

const INPUT: u32 = 1362;

fn is_open(x: u32, y: u32) -> bool {
    let f = x*x + 3*x + 2*x*y + y + y*y + INPUT;
    let b = format!("{:b}", f);
    let sum: u32 = b.chars().map(|c| c.to_digit(10).unwrap()).sum();
    sum % 2 == 0
}

fn neighbors(node: Node) -> Vec<Node> {
    let mut result = Vec::new();
    let (x, y) = node;

    if x > 0 && is_open(x-1, y) { result.push((x-1, y)); }
    if is_open(x+1, y) { result.push((x+1, y)); }

    if y > 0 && is_open(x, y-1) { result.push((x, y-1)); }
    if is_open(x, y+1) { result.push((x, y+1)); }

    result
}

fn part_one() {

    let mut distances: HashMap<Node, u32> = HashMap::new();
    let mut parents: HashMap<Node, Node> = HashMap::new();


    for y in 0..51 {
        for x in 0..51 {
            if is_open(x, y) {
                distances.insert((x, y), u32::max_value());
            }
        }
    }

    let mut q: Vec<Node> = Vec::new();

    let start = (1, 1);

    distances.insert(start, 0);
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();
        for n in neighbors(current) {
            let current_dist = distances.get(&current).unwrap().clone();
            if let Some(dist) = distances.get_mut(&n) {
                if *dist == u32::max_value() {
                    *dist = current_dist + 1;
                    parents.insert(n, current);
                    q.insert(0, n);
                }
            }
        }
    }

    let mut locations_under_50 = 0;

    for dist in distances.values() {
        if *dist <= 50 {
            locations_under_50 += 1;
        }
    }

    println!("Shortest distance: {}", distances.get(&(31, 39)).unwrap());
    println!("Locations nearer than 50 steps: {}", locations_under_50);
}

fn part_two() {
    println!("Not yet implemented");
}
