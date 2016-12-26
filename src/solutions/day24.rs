use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

const INPUT_FILE: &'static str = "input/day24.txt";
const WIDTH: usize = 181;
const HEIGHT: usize = 43;
const GOALS: u32 = 8;

type Grid = [[bool; WIDTH]; HEIGHT];
type Node = (usize, usize);

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let f = File::open(INPUT_FILE).unwrap();
    let reader = BufReader::new(f);

    let mut walls: Grid = [[false; WIDTH]; HEIGHT];
    let mut goals: HashMap<u32, Node> = HashMap::new();

    println!("Initializing wall grid...");
    let mut x = 0;
    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars() {
            match c {
                '#' => {
                    walls[y][x] = true;
                },
                '.' => { },
                d @ _ => {
                    let n = d.to_digit(10).expect("This should be a digit");
                    goals.insert(n, (x, y));
                },
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    let mut paths: HashMap<(u32, u32), u32> = HashMap::new();

    println!("Finding shortest paths...");
    for a in 0..GOALS {
        for b in 0..GOALS {
            if paths.contains_key(&(a, b)) || paths.contains_key(&(b, a)) {
                continue;
            }
            let a_coord = goals.get(&a).unwrap();
            let b_coord = goals.get(&b).unwrap();
            let path = astar_shortest_path(a_coord, b_coord, walls).unwrap();
            paths.insert((a, b), path);
            println!("{} -> {}: {}", a, b, path);
        }
    }

    let mut perms = Vec::new();
    let mut keys = (1..GOALS).collect();
    let r = (GOALS-2) as usize;
    permute(&mut perms, &mut keys, 0, r);

    let mut shortest_path = u32::max_value();

    for p in &mut perms {
        let mut path = 0;
        let mut prev = 0;
        // Uncomment this line for part 2
        // p.push(0);
        for i in 0..p.len() {
            if let Some(length) = paths.get(&(prev, p[i])) {
                path += *length;
            } else {
                path += *paths.get(&(p[i], prev)).unwrap();
            }

            prev = p[i];
        }
        println!("{:?}: {}", p, path);

        if path < shortest_path {
            shortest_path = path;
        }
    }

    println!("Answer: {}", shortest_path);
}

fn permute(perms: &mut Vec<Vec<u32>>, a: &mut Vec<u32>, l: usize, r: usize) {
    if l == r {
        perms.push(a.clone());
    } else {
        for i in l..r+1 {
            a.swap(l, i);
            permute(perms, a, l+1, r);
            a.swap(l, i);
        }
    }
}

#[allow(dead_code)]
fn find_shortest_path(start: &Node, goal: &Node, walls: Grid) -> Option<u32> {
    let mut q: Vec<(u32, Node)> = Vec::new();
    let mut visited: HashSet<Node> = HashSet::new();

    q.insert(0, (0, (start.0, start.1)));

    while !q.is_empty() {
        let (steps, (x, y)) = q.pop().unwrap();
        visited.insert((x, y));
        if (x, y) == (goal.0, goal.1) {
            return Some(steps);
        }
        for n in neighbors(x as usize, y as usize, walls) {
            if !visited.contains(&n) {
                q.insert(0, (steps+1, n));
            }
        }
    }
    None
}

fn manhattan(_: &Node, _: &Node) -> u32 {
    // This manhattan distance calculator is for some reason not needed
    42
}

fn astar_shortest_path(start: &Node, goal: &Node, walls: Grid) -> Option<u32> {
    let mut closed_set: HashSet<Node> = HashSet::new();

    // Should be sorted on fscore
    // let mut open_set: HashSet<Node> = HashSet::new();
    let mut open_set: Vec<Node> = Vec::new();
    open_set.insert(0, *start);

    // Real distance
    let mut g_score: HashMap<Node, u32> = HashMap::new();
    g_score.insert(*start, 0);

    // Heuristic distance
    let mut f_score: HashMap<Node, u32> = HashMap::new();
    f_score.insert(*start, manhattan(start, goal));

    while !open_set.is_empty() {
        // This should be the node with lowest f_score
        let current = open_set.pop().unwrap();
        closed_set.insert(current);

        let mut current_g = u32::max_value();
        if let Some(g) = g_score.get(&current) {
            current_g = *g;
        }

        if current == *goal {
            return Some(current_g);
        }

        let (x, y) = current;
        for neighbor in neighbors(x as usize, y as usize, walls) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let mut neighbor_g = u32::max_value();
            if let Some(g) = g_score.get(&neighbor) {
                neighbor_g = *g;
            }

            let tent_score = current_g + 1;

            if !open_set.contains(&neighbor) {
                open_set.insert(0, neighbor);
            } else if tent_score >= neighbor_g {
                continue;
            }

            g_score.insert(neighbor, tent_score);
            f_score.insert(neighbor, tent_score + manhattan(&neighbor, goal));
        }
    }

    None
}

fn neighbors(x: usize, y: usize, walls: Grid) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();
    if x > 0 && !walls[y][x-1] { result.push((x-1, y)); }
    if x < WIDTH && !walls[y][x+1] { result.push((x+1, y)); }
    if y > 0 && !walls[y-1][x] { result.push((x, y-1)); }
    if y < HEIGHT && !walls[y+1][x] { result.push((x, y+1)); }
    result
}

fn part_two() {
    println!("Not yet implemented");
}
