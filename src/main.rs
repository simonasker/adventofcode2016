mod solutions;

extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of code 2016")
        .version("0.1")
        .author("Simon Andersson <simonasker@posteo.net>")
        .about("Solutions to Advent of code 2016")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("The day to run")
            .required(true)
            .takes_value(true))
        .get_matches();

    let day = matches.value_of("day").unwrap().parse::<i32>().unwrap();

    solutions::run(day);
}
