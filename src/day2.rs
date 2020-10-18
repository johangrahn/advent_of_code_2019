use crate::intcode::Intcode;

pub fn day2(input: &[String]) -> (i64, i64) {
    let mut input: Vec<i64> = input_to_list_of_numbers(&input);
    (part1(&mut input.clone()), part2(&mut input))
}

fn part1(input: &mut Vec<i64>) -> i64 {
    input[1] = 12;
    input[2] = 2;

    let result = Intcode::run(input);
    result[0]
}

fn part2(input: &mut Vec<i64>) -> i64 {
    input[1] = 12;
    input[2] = 2;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut input = input.clone();
            input[1] = noun;
            input[2] = verb;

            let result = Intcode::run(&mut input);
            if result[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn input_to_list_of_numbers(input: &[String]) -> Vec<i64> {
    input[0]
        .split(',')
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::part2;
    use super::{input_to_list_of_numbers, part1};
    use crate::util::read_data;

    #[test]
    fn test_part1() {
        let input = read_data("input/2.txt").unwrap();
        let mut input = input_to_list_of_numbers(&input);
        let result = part1(&mut input);
        assert_eq!(result, 3224742)
    }

    #[test]
    fn test_part2() {
        let input = read_data("input/2.txt").unwrap();
        let mut input = input_to_list_of_numbers(&input);
        let result = part2(&mut input);
        assert_eq!(result, 7960)
    }
}
