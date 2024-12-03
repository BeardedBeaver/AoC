pub mod part1 {
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

    fn get_mul_commands(line: &str) -> Vec<(i64, i64)> {
        let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let captures = re.captures_iter(line);
        let mut result = Vec::new();
        for cap in captures {
            result.push((aoc::parse_or_panic(&cap[1]), aoc::parse_or_panic(&cap[2])));
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
}

pub mod part2 {
    #[derive(Debug, PartialEq, Eq)]
    enum Command {
        Mul(i64, i64),
        Do,
        DoNot,
    }

    fn execute(commands: &[Command]) -> i64 {
        let mut active = true;
        let mut result = 0;
        for c in commands.iter() {
            match *c {
                Command::Mul(a, b) => {
                    if active {
                        result += a * b
                    }
                }
                Command::Do => active = true,
                Command::DoNot => active = false,
            }
        }
        result
    }

    fn parse_commands(line: &str) -> Vec<Command> {
        return vec![];
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut commands = Vec::new();
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let line_commands = parse_commands(&line);
                commands.extend(line_commands);
            }
            execute(&commands).to_string()
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
        use crate::day_03::part2::execute;

        use super::{parse_commands, Command};

        #[test]
        fn test_execute_commands() {
            let commands = vec![
                Command::Mul(4, 6),
                Command::Mul(3, 7),
                Command::DoNot,
                Command::Mul(1, 3),
                Command::Mul(5, 6),
                Command::Do,
                Command::Mul(7, 3),
            ];

            assert_eq!(execute(&commands), 4 * 6 + 3 * 7 + 3 * 7);
        }

        #[test]
        fn test_parse_commands() {
            let line = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
            let commands = parse_commands(line);
            assert_eq!(
                commands,
                vec![
                    Command::Mul(2, 4),
                    Command::DoNot,
                    Command::Mul(5, 5),
                    Command::Mul(11, 8),
                    Command::Do,
                    Command::Mul(8, 5)
                ]
            );

            assert_eq!(execute(&commands), 48);
        }
    }
}
