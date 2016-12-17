pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn dragon(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        output.push(c);
    }

    output.push('0');

    for c in input.chars().rev() {
        match c {
            '0' => output.push('1'),
            '1' => output.push('0'),
            _ => panic!("Input should only contain ones and zeroes")
        }
    }

    output
}

fn part_one() {
    let disk_size = 20;
    let mut input = String::from("10000");

    println!("- Dragon curve");
    loop {
        input = dragon(&input);
        println!("{}", input);

        if input.len() > disk_size {
            break;
        }
    }

    println!("- Truncate to {}", disk_size);
    input.truncate(disk_size);

    println!("{}", input);
}

fn part_two() {
    println!("Not yet implemented");
}
