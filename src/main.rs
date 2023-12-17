use std::{env, process::exit};

mod solutions;
mod utils;

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
    let trimmed = input.trim();

    match day {
        1 => solutions::day01::main(trimmed),
        2 => solutions::day02::main(trimmed),
        3 => solutions::day03::main(trimmed),
        4 => solutions::day04::main(trimmed),
        5 => solutions::day05::main(trimmed),
        6 => solutions::day06::main(trimmed),
        7 => solutions::day07::main(trimmed),
        8 => solutions::day08::main(trimmed),
        9 => solutions::day09::main(trimmed),
        10 => solutions::day10::main(trimmed),
        11 => solutions::day11::main(trimmed),
        12 => solutions::day12::main(trimmed),
        13 => solutions::day13::main(trimmed),
        14 => solutions::day14::main(trimmed),
        15 => solutions::day15::main(trimmed),
        16 => solutions::day16::main(trimmed),
        17 => solutions::day17::main(trimmed),
        _ => unimplemented!(),
    }
}
