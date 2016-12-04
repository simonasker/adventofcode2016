mod day01;
mod day02;

pub fn run(day: i32, part: i32) {
    match day {
        1 => day01::run(part),
        2 => day02::run(part),
        d @ _ => println!("Day {} is not yet implemented", d),
    }
}
