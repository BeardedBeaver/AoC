#[derive(Debug, Default)]
struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Input {
    fn add_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(" ").filter(|i| !i.is_empty()).collect();
        assert!(parts.len() == 2);
        let x1 = aoc::parse_or_panic::<i32>(parts[0]);
        let x2 = aoc::parse_or_panic::<i32>(parts[1]);
        self.left.push(x1);
        self.right.push(x2);
    }
}

pub mod part1 {
    use super::Input;
    use std::iter::zip;

    fn solve(input: &mut Input) -> u64 {
        let mut result = 0;

        input.left.sort();
        input.right.sort();

        for (first, second) in zip(input.left.iter(), input.right.iter()) {
            result += (first - second).abs() as u64;
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut input = Input::default();
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                input.add_line(line);
            }
            solve(&mut input).to_string()
        }

        fn day() -> i32 {
            1
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
        use super::*;

        #[test]
        fn solve_test() {
            let mut input = Input::default();
            input.add_line("3   4");
            input.add_line("4   3");
            input.add_line("2   5");
            input.add_line("1   3");
            input.add_line("3   9");
            input.add_line("3   3");

            assert_eq!(solve(&mut input), 11);
        }
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::Input;

    fn solve(input: &mut Input) -> u64 {
        let mut counts: HashMap<i32, i32> = HashMap::default();

        // count how many time each element occurs in the right list
        for i in input.right.iter() {
            *counts.entry(*i).or_insert(0) += 1;
        }

        // loop through the left list and compute the similarity score
        let mut result: u64 = 0;
        for i in input.left.iter() {
            let count = *counts.entry(*i).or_default() as u64;
            result += count * *i as u64;
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut input = Input::default();
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                input.add_line(line);
            }
            solve(&mut input).to_string()
        }

        fn day() -> i32 {
            1
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }

    mod tests {
        #[test]
        fn solve_test() {
            let mut input = crate::day_01::Input::default();
            input.add_line("3   4");
            input.add_line("4   3");
            input.add_line("2   5");
            input.add_line("1   3");
            input.add_line("3   9");
            input.add_line("3   3");

            assert_eq!(crate::day_01::part2::solve(&mut input), 31);
        }
    }
}
