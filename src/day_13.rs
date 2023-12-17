#[derive(Default)]
struct Pattern {
    cells: Vec<Vec<char>>,
}

impl Pattern {
    fn rotated(&self) -> Pattern {
        let mut rotated_cells = vec![vec![' '; self.cells.len()]; self.cells[0].len()];

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                rotated_cells[j][self.cells.len() - 1 - i] = self.cells[i][j];
            }
        }

        Pattern { cells: rotated_cells }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotated_test() {
        let mut pattern = Pattern::default();
        pattern.cells.push(vec!['.', '#', '#']);
        pattern.cells.push(vec!['#', '.', '.']);

        let rotated = pattern.rotated();
        assert_eq!(3, rotated.cells.len());

        assert_eq!(vec!['#', '.'], rotated.cells[0]);
        assert_eq!(vec!['.', '#'], rotated.cells[1]);
        assert_eq!(vec!['.', '#'], rotated.cells[2]);
    }
}

pub mod part1 {
    use super::*;

    fn is_pattern_reflected(p: &Pattern, line: usize) -> bool {
        let mut top = line;
        let mut bot = line + 1;

        loop {
            if top == 0 {
                break;
            }
            if bot == p.cells.len() - 1 {
                break;
            }
            top -= 1;
            bot += 1;
            if p.cells[top] != p.cells[bot] {
                return false;
            }
        }

        return true;
    }

    // solve a horizontal pattern, if no answer found, rotate
    // a pattern and try again
    fn solve_pattern(p: &Pattern) -> u64 {
        for i in 0..p.cells.len() - 1 {
            if p.cells[i] == p.cells[i + 1] {
                if is_pattern_reflected(p, i) {
                    return 100 * (i + 1) as u64;
                }
            }
        }

        // rotate and try again
        let p = p.rotated();
        for i in 0..p.cells.len() - 1 {
            if p.cells[i] == p.cells[i + 1] {
                if is_pattern_reflected(&p, i) {
                    return (i + 1) as u64;
                }
            }
        }
        unreachable!()
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_pattern_test() {
            let mut result = 0;

            let mut pattern = Pattern::default();
            pattern.cells.push(vec!['#', '.', '#', '#', '.', '.', '#', '#', '.']);
            pattern.cells.push(vec!['.', '.', '#', '.', '#', '#', '.', '#', '.']);
            pattern.cells.push(vec!['#', '#', '.', '.', '.', '.', '.', '.', '#']);
            pattern.cells.push(vec!['#', '#', '.', '.', '.', '.', '.', '.', '#']);
            pattern.cells.push(vec!['.', '.', '#', '.', '#', '#', '.', '#', '.']);
            pattern.cells.push(vec!['.', '.', '#', '#', '.', '.', '#', '#', '.']);
            pattern.cells.push(vec!['#', '.', '#', '.', '#', '#', '.', '#', '.']);

            result += solve_pattern(&pattern);

            let mut pattern = Pattern::default();

            pattern.cells.push(vec!['#', '.', '.', '.', '#', '#', '.', '.', '#']);
            pattern.cells.push(vec!['#', '.', '.', '.', '.', '#', '.', '.', '#']);
            pattern.cells.push(vec!['.', '.', '#', '#', '.', '.', '#', '#', '#']);
            pattern.cells.push(vec!['#', '#', '#', '#', '#', '.', '#', '#', '.']);
            pattern.cells.push(vec!['#', '#', '#', '#', '#', '.', '#', '#', '.']);
            pattern.cells.push(vec!['.', '.', '#', '#', '.', '.', '#', '#', '#']);
            pattern.cells.push(vec!['#', '.', '.', '.', '.', '#', '.', '.', '#']);

            result += solve_pattern(&pattern);

            assert_eq!(405, result);
        }
    }
}
