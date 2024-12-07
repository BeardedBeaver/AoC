#[derive(Debug, PartialEq, Eq)]
struct Equation {
    value: u64,
    operands: Vec<u64>,
}

enum Operation {
    Add,
    Mul,
    Concat,
}

fn parse_equation(line: &str) -> Equation {
    let parts = line.split(':').collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let operands = parts[1].split(" ").filter(|i| !i.is_empty()).collect::<Vec<&str>>();
    Equation {
        value: aoc::parse_or_panic(parts[0]),
        operands: operands.iter().map(|s| aoc::parse_or_panic(s)).collect(),
    }
}

#[cfg(test)]
mod tests {

    use super::{parse_equation, Equation};

    #[test]
    fn test_parse_equation() {
        assert_eq!(
            parse_equation("190: 10 19"),
            Equation {
                value: 190,
                operands: vec![10, 19]
            }
        );
        assert_eq!(
            parse_equation("3267: 81 40 27"),
            Equation {
                value: 3267,
                operands: vec![81, 40, 27]
            }
        );
        assert_eq!(
            parse_equation("83: 17 5"),
            Equation {
                value: 83,
                operands: vec![17, 5]
            }
        );
        assert_eq!(
            parse_equation("156: 15 6"),
            Equation {
                value: 156,
                operands: vec![15, 6]
            }
        );
        assert_eq!(
            parse_equation("7290: 6 8 6 15"),
            Equation {
                value: 7290,
                operands: vec![6, 8, 6, 15]
            }
        );
        assert_eq!(
            parse_equation("161011: 16 10 13"),
            Equation {
                value: 161011,
                operands: vec![16, 10, 13]
            }
        );
        assert_eq!(
            parse_equation("192: 17 8 14"),
            Equation {
                value: 192,
                operands: vec![17, 8, 14]
            }
        );
        assert_eq!(
            parse_equation("21037: 9 7 18 13"),
            Equation {
                value: 21037,
                operands: vec![9, 7, 18, 13]
            }
        );
        assert_eq!(
            parse_equation("292: 11 6 16 20"),
            Equation {
                value: 292,
                operands: vec![11, 6, 16, 20]
            }
        );
    }
}

pub mod part1 {
    use super::{parse_equation, Equation, Operation};

    fn is_valid(value: u64, operands: &[u64], operation: Operation, target: u64) -> bool {
        if value > target {
            return false;
        }
        let new_value = match operation {
            Operation::Add => value + operands[0],
            Operation::Mul => value * operands[0],
            _ => unreachable!(),
        };

        if operands.len() == 1 {
            return new_value == target;
        }

        is_valid(new_value, &operands[1..], Operation::Add, target)
            || is_valid(new_value, &operands[1..], Operation::Mul, target)
    }

    fn is_valid_equation(equation: &Equation) -> bool {
        is_valid(
            equation.operands[0],
            &equation.operands[1..],
            Operation::Add,
            equation.value,
        ) || is_valid(
            equation.operands[0],
            &equation.operands[1..],
            Operation::Mul,
            equation.value,
        )
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let eq = parse_equation(line);
                if is_valid_equation(&eq) {
                    result += eq.value;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            7
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
        use crate::day_07::{part1::is_valid_equation, Equation};

        #[test]
        fn test_is_valid() {
            let eq = Equation {
                value: 190,
                operands: vec![10, 19],
            };
            assert_eq!(is_valid_equation(&eq), true);

            let eq = Equation {
                value: 3267,
                operands: vec![81, 40, 27],
            };
            assert_eq!(is_valid_equation(&eq), true);

            let eq = Equation {
                value: 83,
                operands: vec![17, 5],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 156,
                operands: vec![15, 6],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 7290,
                operands: vec![6, 8, 6, 15],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 161011,
                operands: vec![16, 10, 13],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 192,
                operands: vec![17, 8, 14],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 21037,
                operands: vec![9, 7, 18, 13],
            };
            assert_eq!(is_valid_equation(&eq), false);

            let eq = Equation {
                value: 292,
                operands: vec![11, 6, 16, 20],
            };
            assert_eq!(is_valid_equation(&eq), true);
        }
    }
}

pub mod part2 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
        }

        fn day() -> i32 {
            todo!();
        }

        fn part() -> i32 {
            todo!();
        }

        fn year() -> i32 {
            todo!();
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test() {}
    }
}
