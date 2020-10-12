mod day1;

use std::io::Read;
use std::{collections::HashMap, env, fs::File, io::Error};

type DayFunction = fn(Vec<String>) -> (i64, i64);

fn read_data(input_file: String) -> Result<Vec<String>, Error> {
    let mut input = String::new();

    let mut file = File::open(input_file)?;
    file.read_to_string(&mut input)?;
    let lines = input.split('\n').map(|s: &str| s.to_string()).collect();
    Ok(lines)
}
fn main() {
    let mut solutions: HashMap<usize, DayFunction> = HashMap::new();
    solutions.insert(1, day1::day1);

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

    let input = match read_data(format!("input/{}.txt", day)) {
        Ok(input) => input,
        Err(e) => panic!("Can't read file: {}", e),
    };

    let (part1, part2) = day_function(input);

    println!("Running day {}", day);
    println!("Part1: {}, Part2: {}", part1, part2)
}
