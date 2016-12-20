use std::collections::HashMap;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

const ELVES: i32 = 3018458;
//const ELVES: i32 = 5;

fn part_one() {
    let mut elves_left = ELVES;

    let mut presents: HashMap<i32, i32> = HashMap::new();

    for i in 0..ELVES {
        presents.insert(i, 1);
    }

    let mut i = 0;
    loop {
        println!("{}", elves_left);
        match presents.get(&i) {
            None => { },
            Some(&0) => { },
            Some(&ELVES) => {
                println!("Answer: {}", i+1);
                break;
            },
            Some(_) => {

                let mut steps = elves_left / 2;
                let mut next_elf = i;

                while steps > 0 {
                    next_elf = (next_elf + 1) % ELVES;

                    if presents.get(&next_elf).unwrap_or(&0) > &0 {
                        steps -= 1;
                    }
                }

                let mut num_presents;

                {
                    let next = presents.get_mut(&next_elf).unwrap();
                    num_presents = *next;
                    *next = 0;
                    elves_left -= 1;
                }

                let current = presents.get_mut(&i).unwrap();
                *current += num_presents;
            },
        }

        i = (i + 1) % ELVES;
    }
}

fn part_two() {
    println!("Not yet implemented");
}
