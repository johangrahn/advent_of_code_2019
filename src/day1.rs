pub fn day1(input: Vec<String>) -> (i64, i64) {
    let input: Vec<i64> = input
        .iter()
        .map(|i| i.parse())
        .filter_map(Result::ok)
        .collect();
    (part1(input.clone()), part2(input))
}

fn part1(input: Vec<i64>) -> i64 {
    input.iter().map(calc_fuel).sum()
}

fn part2(input: Vec<i64>) -> i64 {
    input.iter().map(calc_total_fuel).sum()
}

fn calc_fuel(input: &i64) -> i64 {
    (input / 3) - 2
}

fn calc_total_fuel(input: &i64) -> i64 {
    let mut total_consumption = 0;

    let mut tmp = *input;
    loop {
        tmp = calc_fuel(&tmp);

        if tmp <= 0 {
            break;
        }
        total_consumption += tmp;
    }

    total_consumption
}

#[cfg(test)]
mod tests {
    use super::{calc_fuel, calc_total_fuel, part1, part2};

    const INPUT: [i64; 100] = [
        130676, 85676, 100556, 87693, 123830, 80912, 138679, 54162, 51866, 86617, 109838, 59043,
        134132, 96531, 120194, 70404, 72361, 76161, 119764, 121228, 86196, 61936, 147793, 69226,
        70059, 130473, 146038, 62889, 78648, 141921, 146270, 132600, 61658, 141392, 89173, 53501,
        94835, 130408, 58427, 95394, 149591, 60199, 59829, 71011, 119922, 116359, 54330, 68431,
        79188, 52061, 75151, 146200, 74022, 128589, 51944, 134746, 114670, 57787, 104051, 118206,
        84622, 133143, 95292, 123703, 144581, 133101, 104711, 66849, 131474, 81989, 121964, 52866,
        69993, 137283, 128549, 111680, 97570, 115016, 53024, 115880, 112085, 72821, 61449, 145167,
        50947, 98655, 55298, 86164, 99636, 135613, 135293, 97938, 63816, 143939, 58524, 100805,
        61520, 121312, 70638, 117762,
    ];
    #[test]
    fn test_calculator() {
        let input = 12;
        let result = calc_fuel(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculator_big_number() {
        let input = 100756;
        let result = calc_fuel(&input);
        assert_eq!(result, 33583);
    }

    #[test]
    fn test_calculate_total_fuel() {
        let input = 100756;
        let result = calc_total_fuel(&input);
        assert_eq!(result, 50346);
    }

    #[test]
    fn test_calculate_total_fuel_2() {
        let input = 1969;
        let result = calc_total_fuel(&input);
        assert_eq!(result, 966);
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_vec());
        assert_eq!(result, 3249140);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_vec());
        assert_eq!(result, 4870838);
    }
}
