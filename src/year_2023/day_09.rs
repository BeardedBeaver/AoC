use aoc::parse_or_panic;

fn parse_input_file(file_name: &str) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        let data: Vec<i64> = line.split_ascii_whitespace().map(|v| parse_or_panic(v)).collect();
        result.push(data);
    }
    result
}

pub mod part1 {
    use super::*;

    fn solve_line(line: &Vec<i64>) -> i64 {
        if line.iter().all(|x| *x == 0) {
            return 0;
        }
        let mut delta = Vec::new();
        // TODO rework using array_windows
        for i in 0..line.len() - 1 {
            delta.push(line[i + 1] - line[i]);
        }
        return line.last().unwrap() + solve_line(&delta);
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let lines = parse_input_file(file_name);
            let mut result = 0;
            for line in lines.iter() {
                result += solve_line(&line);
            }
            result.to_string()
        }

        fn day() -> i32 {
            9
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn solve_line_test() {
            assert_eq!(28, solve_line(&vec![1, 3, 6, 10, 15, 21]));
            assert_eq!(68, solve_line(&vec![10, 13, 16, 21, 30, 45]));
        }
    }
}

pub mod part2 {
    use super::*;

    fn solve_line(line: &Vec<i64>) -> i64 {
        if line.iter().all(|x| *x == 0) {
            return 0;
        }
        let mut delta = Vec::new();
        for i in 0..line.len() - 1 {
            delta.push(line[i + 1] - line[i]);
        }
        return line.first().unwrap() - solve_line(&delta);
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let lines = parse_input_file(file_name);
            let mut result = 0;
            for line in lines.iter() {
                result += solve_line(&line);
            }
            result.to_string()
        }

        fn day() -> i32 {
            9
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn solve_line_test() {
            assert_eq!(0, solve_line(&vec![1, 3, 6, 10, 15, 21]));
            assert_eq!(5, solve_line(&vec![10, 13, 16, 21, 30, 45]));
        }
    }
}
