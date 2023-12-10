pub mod part1 {
    // Accepts a line and retrieves the first and
    // the last digit from this string
    fn get_value_from_line(line: &str) -> u64 {
        let first = line.chars().find(|c: &char| c.is_digit(10)).unwrap();
        let last = line.chars().rev().find(|c: &char| c.is_digit(10)).unwrap();
        return first.to_digit(10).unwrap() as u64 * 10 + last.to_digit(10).unwrap() as u64;
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(input_file_name: &str) -> String {
            let mut result: u64 = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                result += get_value_from_line(line);
            }
            result.to_string()
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_value_from_line_test() {
            assert_eq!(15, get_value_from_line("a1b2c3d4e5f"));
            assert_eq!(12, get_value_from_line("ojh1nhjybg2hj"));
            assert_eq!(77, get_value_from_line("treb7uchet"));
        }
    }
}

pub mod part2 {
    static DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    fn get_first_digit(line: &str) -> u64 {
        let mut current_line = line;
        loop {
            // first we check for words
            for (index, word) in DIGITS.iter().enumerate() {
                if current_line.starts_with(*word) {
                    return index as u64 + 1;
                }
            }

            // then we check if it starts with a digit
            let c = current_line.as_bytes()[0] as char;
            let maybe_value = c.to_digit(10);
            match maybe_value {
                Some(value) => {
                    return value as u64;
                }
                _ => (),
            }

            // if not found, strip the first character and repeat
            current_line = &current_line[1..];
        }
    }

    fn get_last_digit(line: &str) -> u64 {
        let mut current_line = line;
        loop {
            // first we check for words
            for (index, word) in DIGITS.iter().enumerate() {
                if current_line.ends_with(*word) {
                    return index as u64 + 1;
                }
            }

            // then we check if it starts with a digit
            let c = current_line.as_bytes()[current_line.len() - 1] as char;
            let maybe_value = c.to_digit(10);
            match maybe_value {
                Some(value) => {
                    return value as u64;
                }
                _ => (),
            }

            // if not found, strip the first character and repeat
            current_line = &current_line[..current_line.len() - 1];
        }
    }

    // Accepts a line and retrieves the first and
    // the last digit from this string
    fn get_value_from_line(line: &str) -> u64 {
        return get_first_digit(line) * 10 + get_last_digit(line);
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(input_file_name: &str) -> String {
            let mut result: u64 = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                result += get_value_from_line(line);
            }
            result.to_string()
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_value_from_line_test() {
            assert_eq!(29, get_value_from_line("two1nine"));
            assert_eq!(83, get_value_from_line("eightwothree"));
            assert_eq!(13, get_value_from_line("abcone2threexyz"));
            assert_eq!(24, get_value_from_line("xtwone3four"));
            assert_eq!(42, get_value_from_line("4nineeightseven2"));
            assert_eq!(14, get_value_from_line("zoneight234"));
            assert_eq!(76, get_value_from_line("7pqrstsixteen"));
        }
    }
}
