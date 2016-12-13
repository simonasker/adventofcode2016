use std::collections::HashMap;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

type Node = (u32, u32);

// const INPUT: u32 = 1362;
const INPUT: u32 = 10;

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
    let mut parents: HashMap<Node, Option<Node>> = HashMap::new();

    for y in 0..40 {
        for x in 0..40 {
            if is_open(x, y) {
                distances.insert((x, y), u32::max_value());
                parents.insert((x, y), None);
            }

            // let sign = match is_open(x, y) {
            //     true => '.',
            //     false => '#',
            // };
            // print!("{}", sign);
        }
        // print!("\n");
    }

    let mut q: Vec<Node> = Vec::new();

    let start = (1, 1);

    distances.insert(start, 0);
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();
        println!("current: {:?}", current);
        for n in neighbors(current) {
            println!("n: {:?}", n);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
