use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

#[derive(Clone, Debug)]
struct State {
    state: [u32; STATE_ENTRIES],
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        let mut self_comp = vec![];
        let mut other_comp = vec![];

        let mut i = 0;
        while i < STATE_ENTRIES - 1 {
            self_comp.push((self.state[i+1], self.state[i+2]));
            other_comp.push((other.state[i+1], other.state[i+2]));
            i += 2;
        }

        self_comp.sort();
        other_comp.sort();

        self.state[0] == other.state[0] && self_comp == other_comp
    }
}

impl Eq for State { }

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut self_comp = vec![];
        let mut i = 0;
        while i < STATE_ENTRIES - 1 {
            self_comp.push((self.state[i+1], self.state[i+2]));
            i += 2;
        }
        self_comp.sort();
        self.state[0].hash(state);
        self_comp.hash(state);
    }
}

fn is_done(state: &State) -> bool {
    for i in 1..STATE_ENTRIES {
        if state.state[i] != 3 {
            return false;
        }
    }
    true
}

fn is_valid(state: &State) -> bool {
    let mut i = 2;
    while i < STATE_ENTRIES {

        // If microchip doesn't have it's generator
        if state.state[i] != state.state[i-1] {

            let mut j = 1;
            while j < STATE_ENTRIES {
                if state.state[j] == state.state[i] {
                    return false;
                }
                j += 2;
            }

        }
        i += 2;
    }
    true
}

fn components_below(state: &State, elevator: u32) -> bool {
    for i in 0..STATE_ENTRIES {
        if state.state[i] < elevator {
            return true;
        }
    }
    false
}

fn neighbors(state: State) -> Vec<State> {
    let mut state_set = HashSet::new();

    let elevator = state.state[0];

    if elevator < 3 {
        for i in 1..STATE_ENTRIES {
            if state.state[i] == elevator {
                let mut new_state = state.clone();
                new_state.state[0] += 1;
                new_state.state[i] += 1;
                state_set.insert(new_state.clone());
                for j in 1..STATE_ENTRIES {
                    if new_state.state[j] == elevator {
                        let mut new_state_2 = new_state.clone();
                        new_state_2.state[j] += 1;
                        state_set.insert(new_state_2.clone());
                    }
                }
            }
        }
    }

    // TODO Fucking ugly way to handle going down as well
    if elevator > 0 && components_below(&state, elevator) {
        for i in 1..STATE_ENTRIES {
            if state.state[i] == elevator {
                let mut new_state = state.clone();
                new_state.state[0] -= 1;
                new_state.state[i] -= 1;
                state_set.insert(new_state.clone());
                for j in 1..STATE_ENTRIES {
                    if new_state.state[j] == elevator {
                        let mut new_state_2 = new_state.clone();
                        new_state_2.state[j] -= 1;
                        state_set.insert(new_state_2.clone());
                    }
                }
            }
        }
    }

    // TODO Would be nicer to return an iterator directly
    state_set.into_iter().filter(|s| is_valid(s)).collect()
}

// const STATE_ENTRIES: usize = 11; // Part 1
const STATE_ENTRIES: usize = 15; // Part 2

fn part_one() {

    let start = State {
        // state: [0, 0, 0, 1, 2, 1, 2, 1, 2, 1, 2], // Part 1
        state: [0, 0, 0, 1, 2, 1, 2, 1, 2, 1, 2, 0, 0, 0, 0], // Part 2
    };

    let mut visited: HashSet<State> = HashSet::new();
    let mut steps: HashMap<State, u32> = HashMap::new();
    steps.insert(start.clone(), 0);

    let mut q: Vec<State> = Vec::new();
    q.insert(0, start);


    let mut iterations = 0;
    while !q.is_empty() {
        let current = q.pop().unwrap();
        let s = steps.get(&current).unwrap().clone();

        visited.insert(current.clone());

        println!("{:?} {}", current, s);
        if is_done(&current) {
            println!("DONE in {} steps", s);
            break;
        }

        for n in neighbors(current) {
            let mut nstep = u32::max_value();
            if let Some(step) = steps.get(&n) {
                nstep = *step;
            }
            if s + 1 < nstep || (!visited.contains(&n) && !q.contains(&n)) {
                steps.insert(n.clone(), s+1);
                q.insert(0, n.clone());
            }
        }
        iterations += 1;
    }

    println!("Iterations: {}", iterations);
}

fn part_two() {
    println!("Not yet implemented");
}
