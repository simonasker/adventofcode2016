mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn run(day: i32, part: i32) {
    match day {
        1 => day01::run(part),
        2 => day02::run(part),
        3 => day03::run(part),
        4 => day04::run(part),
        5 => day05::run(part),
        6 => day06::run(part),
        d @ _ => println!("Day {} is not yet implemented", d),
    }
}
