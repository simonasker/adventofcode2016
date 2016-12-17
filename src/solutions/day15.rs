pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let start_position = [4, 1];
    let positions = [5, 2];

    'start_time: for start_time in 0.. {
        // TODO Remove this
        // if start_time >= 10 { break; }

        let mut current_positions = [0, 0];

        // Set starting positions
        for i in 0..start_position.len() {
            current_positions[i] = (start_position[i] + start_time) % positions[i];
        }

        // let mut time = start_time;
        let mut ball = 0;

        println!("START TIME: {}", start_time);
        loop {
            // TODO Remove this
            // time += 1;
            // if time == start_time + 5 { break; }

            println!("{:?}", current_positions);
            if current_positions[ball] != 0 {
                println!("Bounced");
                continue 'start_time;
            }

            ball += 1;
            if ball == current_positions.len() {
                println!("Reached the end when starting at time: {}", start_time);
                break 'start_time;
            }

            // Update discs
            for i in 0..current_positions.len() {
                current_positions[i] = (current_positions[i] + 1) % positions[i];
            }
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
