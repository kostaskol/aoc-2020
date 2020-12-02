extern crate clap;
use clap::{Arg, App};

mod utils;
mod p1;
mod p2;

fn main() {
    let matches = App::new("Advent of code!")
        .arg(Arg::with_name("extra")
                .short("e")
                .long("extra")
                .takes_value(false))
        .arg(Arg::with_name("problem_number")
                .short("p")
                .long("problem")
                .takes_value(true))
        .get_matches();

    let extra_star = matches.is_present("extra");
    let problem = matches.value_of("problem_number").unwrap_or("1");

    match problem {
        "1" => p1::run(extra_star)
        &_ => println!("Only know how to solve #1 for now :(")
    }
}