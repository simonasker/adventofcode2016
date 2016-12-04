mod solutions;

extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code 2016")
        .version("0.1")
        .author("Simon Andersson <simonasker@posteo.net>")
        .about("Solutions to Advent of Code 2016")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("The day to run")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("part")
            .short("p")
            .long("part")
            .value_name("PART")
            .help("The part to run")
            .takes_value(true))
        .get_matches();

    let day = matches.value_of("day").unwrap().parse::<i32>().unwrap();
    let part = matches.value_of("part").unwrap_or("1").parse::<i32>().unwrap();

    solutions::run(day, part);
}
