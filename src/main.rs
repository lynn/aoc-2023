use std::{env, process::exit};

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: aoc-2023 [day] [input.txt]");
        exit(1);
    }
    let day: usize = str::parse(&args[1]).expect("couldn't parse day");
    let input = std::fs::read_to_string(&args[2])
        .expect("couldn't read input")
        .replace('\r', "");

    match day {
        1 => solutions::day01::main(input.trim()),
        2 => solutions::day02::main(input.trim()),
        3 => solutions::day03::main(input.trim()),
        4 => solutions::day04::main(input.trim()),
        _ => unimplemented!(),
    }
}
