pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let input = 3018458;
    let mut res = 0;
    for i in 0.. {
        if i32::pow(2, i) > input {
            res = i32::pow(2, (i-1));
            break;
        }
    }
    let answer = (input - res) * 2 + 1;
    println!("Answer: {}", answer);
}

fn part_two() {
    let input = 3018458;
}

fn get_last_elf(num_elves: i32) -> i32 {
    let mut elves: Vec<i32> = Vec::new();

    for i in 1..num_elves+1 {
        elves.push(i);
    }

    let mut i = 0;
    loop {
        // println!("{:?}", elves);
        // println!("Index: {}", i);
        if elves.len() == 1 {
            break;
        }

        let next = (i + 1) % elves.len();
        // println!("Removing elves[{}]: {}", next, elves[next]);
        elves.remove(next);

        if i == elves.len() {
            i -= 1;
        }

        i = (i + 1) % elves.len();

    }

    elves[0]
}

