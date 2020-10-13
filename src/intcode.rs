use crate::opcode::*;
pub struct Intcode {}

impl Intcode {
    pub fn run(input: &mut Vec<i64>) -> &Vec<i64> {
        let mut index = 0;

        loop {
            if index >= (input.len() - 1) {
                break;
            }

            let opcode = parse_opcode(input[index]);

            let first_number = input[input[index + 1] as usize];
            let second_number = input[input[index + 2] as usize];

            let result = match opcode {
                Opcode::Add => first_number + second_number,
                Opcode::Multiply => first_number * second_number,
                Opcode::Quit => break,
            };

            let result_pos = input[index + 3];
            input[result_pos as usize] = result;

            index += 4;
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use super::Intcode;

    #[test]
    fn test_intcode_smal_version() {
        let mut input = vec![1, 0, 0, 0, 99];
        let result = Intcode::run(&mut input);
        assert_eq!(result[0], 2);
    }

    #[test]
    fn test_intcode_bigger_version() {
        let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = Intcode::run(&mut input);
        assert_eq!(result[0], 30);
        assert_eq!(result[4], 2)
    }
}
