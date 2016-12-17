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

    for start_time in 0.. {
        let mut current_positions = [0, 0];

        for i in 0..start_position.len() {
            current_positions[i] = (start_position[i] + start_time) % positions[i];
        }

        println!("{:?}", current_positions);

        if start_time >= 10 {
            break;
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
