pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

const STATE_ENTRIES: usize = 5;

type State = [u32; STATE_ENTRIES];

fn is_valid(state: [u32; STATE_ENTRIES]) -> bool {
    true
}

fn neighbors(state: [u32; STATE_ENTRIES]) -> Vec<[u32; STATE_ENTRIES]> {
    let mut result = Vec::new();

    result
}

fn part_one() {

    let start = [0, 1, 0, 2, 0];

    let mut q: Vec<State> = Vec::new();
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();

        println!("{:?}", current);

        for n in neighbors(current) {
            q.insert(0, n);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
