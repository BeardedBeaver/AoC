#[derive(Default)]
struct Pattern {
    cells: Vec<Vec<char>>,
}

pub mod part1 {
    use super::*;

    fn solve_pattern(p: &Pattern) -> u64 {
        0
    }

    fn solve(patterns: &Vec<Pattern>) -> u64 {
        let mut result = 0;
        for p in patterns.iter() {
            result += solve_pattern(&p);
        }
        result
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut patterns = Vec::new();
            let mut pattern = Pattern::default();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                if line.is_empty() {
                    patterns.push(pattern);
                    pattern = Pattern::default();
                } else {
                    pattern.cells.push(line.chars().collect());
                }
            }
            if !pattern.cells.is_empty() {
                patterns.push(pattern);
            }
            println!("{}", patterns.len());
            solve(&patterns).to_string()
        }

        fn day() -> i32 {
            13
        }

        fn part() -> i32 {
            1
        }
    }
}
