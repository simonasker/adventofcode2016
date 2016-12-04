mod day01;
mod day02;

pub fn run(day: i32) {
    match day {
        1 => day01::run(),
        2 => day02::run(),
        d @ _ => println!("Day {} is not yet implemented", d),
    }
}
