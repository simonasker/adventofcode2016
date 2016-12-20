use std::collections::HashMap;

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

const ELVES: i32 = 3018458;

fn part_one() {

    let mut presents: HashMap<i32, i32> = HashMap::new();

    for i in 0..ELVES {
        presents.insert(i, 1);
    }

    let mut i = 0;
    loop {
        match presents.get(&i) {
            None => { },
            Some(&0) => { },
            Some(&ELVES) => {
                println!("Answer: {}", i+1);
                break;
            },
            Some(_) => {
                let mut next_elf = (i + 1) % ELVES;

                while let Some(&0) = presents.get(&next_elf) {
                    next_elf = (next_elf + 1) % ELVES;
                }

                let mut num_presents;

                {
                    let next = presents.get_mut(&next_elf).unwrap();
                    num_presents = *next;
                    *next = 0;
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
