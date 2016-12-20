pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

const ELVES: usize = 5;
//const ELVES: usize = 3018458;

fn part_one() {
    let mut presents = [1; ELVES];

    let mut i = 0;
    loop {
        match presents[i] {
            0 => { },
            ELVES => {
                println!("Answer: {}", i+1);
                break;
            },
            _ => {
                let mut next_elf = (i + 1) % ELVES;
                while presents[next_elf] == 0 {
                    next_elf = (next_elf + 1) % ELVES;
                }

                let num_presents = presents[next_elf];
                presents[i] += num_presents;
                presents[next_elf] = 0;
            },
        }

        i = (i + 1) % ELVES;
    }

}

fn part_two() {
    println!("Not yet implemented");
}
