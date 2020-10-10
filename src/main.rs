use std::{collections::HashMap, env};

fn day1(input: Vec<String>) -> (i64, i64) {
    (0, 0)
}

type DayFunction = fn(Vec<String>) -> (i64, i64);
fn main() {
    let mut solutions: HashMap<usize, DayFunction> = HashMap::new();
    solutions.insert(1, day1);

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

    let (part1, part2) = day_function(vec![String::from("")]);

    println!("Running day {}", day);
    println!("Part1: {}, Part2: {}", part1, part2)
}
