use aoc::parse_or_panic;

fn get_mul_commands(line: &str) -> Vec<(i64, i64)> {
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let captures = re.captures_iter(line);
    let mut result = Vec::new();
    for cap in captures {
        result.push((parse_or_panic(&cap[1]), parse_or_panic(&cap[2])));
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_mul_commands_test() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::get_mul_commands(line);
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);

        let line = "xmul(2,4567)";
        let result = super::get_mul_commands(line);
        assert!(result.is_empty());
    }
}

pub mod part1 {
    use super::get_mul_commands;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let values = get_mul_commands(line);
                for v in values {
                    result += v.0 * v.1;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            3
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2024
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test() {}
    }
}

pub mod part2 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            return "".to_string();
        }

        fn day() -> i32 {
            3
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test() {}
    }
}
