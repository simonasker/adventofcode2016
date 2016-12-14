mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
        10 => day10::run(part),
        11 => day11::run(part),
        12 => day12::run(part),
        13 => day13::run(part),
        14 => day14::run(part),
        d @ _ => println!("Day {} is not yet implemented", d),
    }
}
