mod day1;
mod day2;
mod intcode;
mod opcode;
mod util;

use std::{collections::HashMap, env};

use util::read_data;

type DayFunction = fn(Vec<String>) -> (i64, i64);

fn main() {
    let mut solutions: HashMap<usize, DayFunction> = HashMap::new();
    solutions.insert(1, day1::day1);
    solutions.insert(2, day2::day2);

    // Get user input
    let args: Vec<String> = env::args().collect();
    let day = match args.get(1) {
        Some(s) => match s.parse::<usize>() {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        },
        None => 1,
    };

    let day_function = match solutions.get(&day) {
        Some(day) => day,
        None => panic!("Can't find day definition for day: {}", day),
    };

    let filename = format!("input/{}.txt", day);
    let input = match read_data(&filename) {
        Ok(input) => input,
        Err(e) => panic!("Can't read file: {} -> {}", filename, e),
    };

    let (part1, part2) = day_function(input);

    println!("Running day {}", day);
    println!("Part1: {}, Part2: {}", part1, part2)
}
