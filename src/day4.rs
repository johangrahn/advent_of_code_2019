use std::thread::current;

pub fn day4(input: &[String]) -> (i64, i64) {
    (part1(input), part2(input))
}

fn part1(input: &[String]) -> i64 {
    let arr = input[0]
        .split("-")
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect::<Vec<i64>>();

    let mut num_ok_passwords = 0;
    for number in arr[0]..=arr[1] {
        match check_number_part1(number) {
            true => num_ok_passwords += 1,
            false => {}
        }
    }
    num_ok_passwords
}

fn part2(input: &[String]) -> i64 {
    let arr = input[0]
        .split("-")
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect::<Vec<i64>>();

    let mut num_ok_passwords = 0;
    for number in arr[0]..=arr[1] {
        match check_number_part2(number) {
            true => num_ok_passwords += 1,
            false => {}
        }
    }

    num_ok_passwords
}

fn check_number_part1(input: i64) -> bool {
    let str_number = input.to_string();
    let mut prev_number = 0;
    let mut found_match = false;

    for n in str_number.chars() {
        let current_number = n.to_digit(10).unwrap();
        if current_number < prev_number {
            return false;
        }

        if prev_number == current_number {
            found_match = true;
        }

        prev_number = current_number;
    }

    found_match
}

enum NumberState {
    None,
    One(u32),
    Two(u32),
    ThreeOrMore(u32),
}

fn check_number_part2(input: i64) -> bool {
    let str_number = input.to_string();
    let mut prev_number = 0;
    let mut found_match = false;
    let mut state = NumberState::None;

    for n in str_number.chars() {
        let current_number = n.to_digit(10).unwrap();

        state = match state {
            NumberState::None => NumberState::One(current_number),
            NumberState::One(digit) => {
                if digit == current_number {
                    NumberState::Two(digit)
                } else if digit < prev_number {
                    return false;
                } else {
                    prev_number = digit;
                    NumberState::One(d)
                }
            }
            NumberState::Two(digit) => {
                if digit != current_number {
                    found_match = true;
                    NumberState::One(current_number)
                } else {
                    NumberState::ThreeOrMore(current_number)
                }
            }
            NumberState::ThreeOrMore(digit) => NumberState::ThreeOrMore(digit),
        };

        // if current_number < prev_number {
        //     return false;
        // }

        // if prev_number == current_number {
        //     found_match = true;
        // }
    }

    found_match
}

fn somesthing() -> () {
    todo!()
}

#[cfg(test)]
mod day4_tests {
    use super::check_number_part1;
    use super::check_number_part2;

    #[test]
    fn day4_part1_test1() {
        assert_eq!(check_number_part1(111111), true);
    }

    #[test]
    fn day4_part1_test2() {
        assert_eq!(check_number_part1(223450), false);
    }

    #[test]
    fn day4_part1_test3() {
        assert_eq!(check_number_part1(123789), false);
    }

    #[test]
    fn day4_part2_test1() {
        assert_eq!(check_number_part2(112233), true);
    }

    #[test]
    fn day4_part2_test2() {
        assert_eq!(check_number_part2(123444), false);
    }

    #[test]
    fn day4_part2_test3() {
        assert_eq!(check_number_part2(111122), true);
    }
}
