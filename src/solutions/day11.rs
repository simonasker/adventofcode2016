use std::collections::HashSet;

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
    let mut state_set = HashSet::new();

    let elevator = state[0];

    for i in 1..STATE_ENTRIES {
        if state[i] == elevator {
            let mut new_state = state.clone();
            new_state[0] += 1;
            new_state[i] += 1;
            state_set.insert(new_state);
            for j in 1..STATE_ENTRIES {
                if new_state[j] == elevator {
                    let mut new_state_2 = new_state.clone();
                    new_state_2[j] += 1;
                    state_set.insert(new_state_2);
                }
            }
        }
    }

    // TODO Filter on valid states
    // TODO Would be nicer to return an iterator directly
    state_set.into_iter().collect()
}

fn part_one() {

    let start = [0, 1, 0, 2, 0];

    let mut q: Vec<State> = Vec::new();
    q.insert(0, start);

    while !q.is_empty() {
        let current = q.pop().unwrap();

        println!("{:?}", current);
        println!("-------------");

        for n in neighbors(current) {
            println!("{:?}", n);
            // q.insert(0, n);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
