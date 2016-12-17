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
    // let disk_size = 20; // example disk size
    // let disk_size = 272; // part 1
    let disk_size = 35651584; // part 2

    // let mut input = String::from("10000"); // example input
    let mut input = String::from("01111001100111011"); // real input

    println!("- Dragon curve");
    loop {
        input = dragon(&input);

        if input.len() > disk_size {
            break;
        }
    }

    println!("- Truncate to {}", disk_size);
    input.truncate(disk_size);

    println!("- Generate checksums");
    loop {
        let mut checksum = String::new();

        {
            let mut char_iter = input.chars().peekable();

            loop {
                let a = char_iter.next().unwrap();
                let b = char_iter.next().unwrap();

                if a == b {
                    checksum.push('1');
                } else {
                    checksum.push('0');
                }

                if let None = char_iter.peek() {
                    break;
                }
            }
        }

        if checksum.len() % 2 != 0 {
            println!("- Final result: {}", checksum);
            break;
        } else {
            input = checksum;
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
