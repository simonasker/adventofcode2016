mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

pub fn run(day: i32, part: i32) {
    match day {
        1 => day01::run(part),
        2 => day02::run(part),
        3 => day03::run(part),
        4 => day04::run(part),
        5 => day05::run(part),
        6 => day06::run(part),
        7 => day07::run(part),
        8 => day08::run(part),
        9 => day09::run(part),
        d @ _ => println!("Day {} is not yet implemented", d),
    }
}
