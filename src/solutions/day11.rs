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

fn is_done(state: State) -> bool {
    for i in 1..STATE_ENTRIES {
        if state[i] != 3 {
            return false;
        }
    }
    true
}

fn is_valid(state: State) -> bool {
    let mut i = 2;
    while i < STATE_ENTRIES {

        // If microchip doesn't have it's generator
        if state[i] != state[i-1] {

            let mut j = 1;
            while j < STATE_ENTRIES {
                if state[j] == state[i] {
                    return false;
                }
                j += 2;
            }

        }
        i += 2;
    }
    true
}

fn neighbors(state: State) -> Vec<State> {
    let mut state_set = HashSet::new();

    let elevator = state[0];

    if elevator < 3 {
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
    }

    // TODO Fucking ugly way to handle going down as well
    if elevator > 0 {
        for i in 1..STATE_ENTRIES {
            if state[i] == elevator {
                let mut new_state = state.clone();
                new_state[0] -= 1;
                new_state[i] -= 1;
                state_set.insert(new_state);
                for j in 1..STATE_ENTRIES {
                    if new_state[j] == elevator {
                        let mut new_state_2 = new_state.clone();
                        new_state_2[j] -= 1;
                        state_set.insert(new_state_2);
                    }
                }
            }
        }
    }

    // TODO Would be nicer to return an iterator directly
    state_set.into_iter().filter(|s| is_valid(*s)).collect()
}

fn part_one() {

    let start = [0, 1, 0, 2, 0];

    let mut visited: HashSet<State> = HashSet::new();

    let mut q: Vec<(u32, State)> = Vec::new();
    q.insert(0, (0, start));

    let mut iterations = 0;

    while !q.is_empty() {
        println!("it: {}", iterations);
        let (steps, current) = q.pop().unwrap();

        visited.insert(current);

        println!("{:?}", current);
        if is_done(current) {
            println!("DONE in {} steps", steps);
            break;
        }

        for n in neighbors(current) {
            if !visited.contains(&n) {
                q.insert(0, (steps+1, n));
            }
        }

        iterations += 1;
    }
}

fn part_two() {
    println!("Not yet implemented");
}
