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

fn get_mul_commands(line: &str) -> Vec<Command> {
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let captures = re.captures_iter(line);
    let mut result = Vec::new();
    for cap in captures {
        result.push(Command::Mul(aoc::parse_or_panic(&cap[1]), aoc::parse_or_panic(&cap[2])));
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day_03::{execute, Command};

    #[test]
    fn get_mul_commands_test() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::get_mul_commands(line);
        assert_eq!(
            result,
            vec![
                Command::Mul(2, 4),
                Command::Mul(5, 5),
                Command::Mul(11, 8),
                Command::Mul(8, 5)
            ]
        );

        let line = "xmul(2,4567)";
        let result = super::get_mul_commands(line);
        assert!(result.is_empty());
    }

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
}

pub mod part1 {
    use super::{get_mul_commands, Command};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let commands = get_mul_commands(line);
                for command in commands {
                    match command {
                        Command::Mul(a, b) => result += a * b,
                        Command::Do => unreachable!(),
                        Command::DoNot => unreachable!(),
                    };
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
}

pub mod part2 {
    use super::{execute, Command};
    use std::num::ParseIntError;

    #[derive(Debug)]
    #[allow(dead_code)]
    enum ParseError {
        InvalidFormat,
        ParseError(ParseIntError),
    }

    impl From<ParseIntError> for ParseError {
        fn from(err: ParseIntError) -> Self {
            ParseError::ParseError(err)
        }
    }

    fn parse_mul_arguments(line: &str) -> Result<(i64, i64), ParseError> {
        let parts = line.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat);
        }
        Ok((parts[0].parse::<i64>()?, parts[1].parse::<i64>()?))
    }

    fn parse_commands(line: &str) -> Vec<Command> {
        let mut line = line;
        let mut result = Vec::new();
        while !line.is_empty() {
            if line.starts_with("do()") {
                result.push(Command::Do);
                line = &line[4..];
            } else if line.starts_with("don't()") {
                result.push(Command::DoNot);
                line = &line[6..];
            } else {
                if line.starts_with("mul(") {
                    if let Some(closing_brace_pos) = line.chars().position(|c| c == ')') {
                        let command_args_str = &line[4..closing_brace_pos];
                        if let Ok(args) = parse_mul_arguments(command_args_str) {
                            result.push(Command::Mul(args.0, args.1));
                            line = &line[closing_brace_pos..];
                        }
                        line = &line[1..];
                    } else {
                        // closing brace not found, can exit early
                        break;
                    }
                } else {
                    line = &line[1..];
                }
            }
        }
        result
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
