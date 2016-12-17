pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

const NUMBER_OF_DISCS: usize = 7;

fn part_one() {
    // let start_position = [0, 4, 1];
    // let positions = [1, 5, 2];
    let start_position = [0, 1, 0, 2, 0, 0, 5];
    let positions = [1, 17, 7, 19, 5, 3, 13];

    'start_time: for start_time in 0.. {

        let mut current_positions = [0; NUMBER_OF_DISCS];
        for i in 0..NUMBER_OF_DISCS {
            current_positions[i] = (start_position[i] + start_time) % positions[i];
        }

        let mut ball = 0;

        println!("\nSTART TIME: {}", start_time);
        loop {

            print!("{:?}", current_positions);
            if current_positions[ball] != 0 {
                println!(" Bounced");
                continue 'start_time;
            }
            println!("");

            ball += 1;
            if ball == NUMBER_OF_DISCS {
                println!("Reached the end when starting at time: {}", start_time);
                break 'start_time;
            }

            // Update discs
            for i in 0..NUMBER_OF_DISCS {
                current_positions[i] = (current_positions[i] + 1) % positions[i];
            }
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
