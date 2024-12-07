#[derive(Debug, PartialEq, Eq)]
struct Equation {
    value: u64,
    operands: Vec<i64>,
}

fn parse_equation(line: &str) -> Equation {
    let parts = line.split(':').collect::<Vec<&str>>();
    let operands = parts[1].split(" ").filter(|i| !i.is_empty()).collect::<Vec<&str>>();
    Equation {
        value: aoc::parse_or_panic(parts[0]),
        operands: operands.iter().map(|s| aoc::parse_or_panic(s)).collect(),
    }
}

fn is_valid(equation: &Equation) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::day_07::is_valid;

    use super::{parse_equation, Equation};

    #[test]
    fn test_parse_equation() {
        let eq = parse_equation("190: 10 19");
        assert_eq!(
            eq,
            Equation {
                value: 190,
                operands: vec![10, 19]
            }
        );
        let eq = parse_equation("3267: 81 40 27");
        assert_eq!(
            eq,
            Equation {
                value: 3267,
                operands: vec![81, 40, 27]
            }
        );
        let eq = parse_equation("83: 17 5");
        assert_eq!(
            eq,
            Equation {
                value: 83,
                operands: vec![17, 5]
            }
        );
        let eq = parse_equation("156: 15 6");
        assert_eq!(
            eq,
            Equation {
                value: 156,
                operands: vec![15, 6]
            }
        );
        let eq = parse_equation("7290: 6 8 6 15");
        assert_eq!(
            eq,
            Equation {
                value: 7290,
                operands: vec![6, 8, 6, 15]
            }
        );
        let eq = parse_equation("161011: 16 10 13");
        assert_eq!(
            eq,
            Equation {
                value: 161011,
                operands: vec![16, 10, 13]
            }
        );
        let eq = parse_equation("192: 17 8 14");
        assert_eq!(
            eq,
            Equation {
                value: 192,
                operands: vec![17, 8, 14]
            }
        );
        let eq = parse_equation("21037: 9 7 18 13");
        assert_eq!(
            eq,
            Equation {
                value: 21037,
                operands: vec![9, 7, 18, 13]
            }
        );
        let eq = parse_equation("292: 11 6 16 20");
        assert_eq!(
            eq,
            Equation {
                value: 292,
                operands: vec![11, 6, 16, 20]
            }
        );
    }

    #[test]
    fn test_is_valid() {
        let eq = Equation {
            value: 190,
            operands: vec![10, 19],
        };
        assert_eq!(is_valid(&eq), true);

        let eq = Equation {
            value: 3267,
            operands: vec![81, 40, 27],
        };
        assert_eq!(is_valid(&eq), true);

        let eq = Equation {
            value: 83,
            operands: vec![17, 5],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 156,
            operands: vec![15, 6],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 7290,
            operands: vec![6, 8, 6, 15],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 161011,
            operands: vec![16, 10, 13],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 192,
            operands: vec![17, 8, 14],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 21037,
            operands: vec![9, 7, 18, 13],
        };
        assert_eq!(is_valid(&eq), false);

        let eq = Equation {
            value: 292,
            operands: vec![11, 6, 16, 20],
        };
        assert_eq!(is_valid(&eq), true);
    }
}

pub mod part1 {
    use super::{is_valid, parse_equation};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let eq = parse_equation(line);
                if is_valid(&eq) {
                    result += 1;
                }
            }
            result.to_string()
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
