pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

// const INPUT: u32 = 1362;
const INPUT: u32 = 10;

fn is_open(x: u32, y: u32) -> bool {
    let f = x*x + 3*x + 2*x*y + y + y*y + INPUT;
    let b = format!("{:b}", f);
    let sum: u32 = b.chars().map(|c| c.to_digit(10).unwrap()).sum();
    sum % 2 == 0
}

fn part_one() {
    for y in 0..40 {
        for x in 0..40 {
            let sign = match is_open(x, y) {
                true => '.',
                false => '#',
            };
            print!("{}", sign);
        }
        print!("\n");
    }
}

fn part_two() {
    println!("Not yet implemented");
}
