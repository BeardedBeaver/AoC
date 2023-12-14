struct Universe {
    galaxies: Vec<String>,
}

impl Universe {
    fn from_file(file_name: &str) -> Universe {
        let mut galaxies: Vec<String> = Vec::new();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            galaxies.push(line.to_owned());
        }
        Universe { galaxies: galaxies }
    }

    fn expand(self: &mut Universe) {
        assert!(!self.galaxies.is_empty());

        // horizontal lines first
        let mut indicies = Vec::new();
        for (i, line) in self.galaxies.iter().enumerate() {
            if line.as_bytes().iter().all(|c| *c == '.' as u8) {
                indicies.push(i);
            }
        }
        for i in indicies.iter().rev() {
            self.galaxies[*i] = "r".repeat(self.galaxies[0].len());
        }

        // and now vertical lines
        let mut indicies = Vec::new();
        for i in 0..self.galaxies[0].len() {
            let mut empty = true;
            for line in self.galaxies.iter() {
                if line.as_bytes()[i] == '#' as u8 {
                    empty = false;
                    break;
                }
            }
            if empty {
                indicies.push(i);
            }
        }
        for i in indicies.iter().rev() {
            let i = *i;
            for line in self.galaxies.iter_mut() {
                let c = line.as_bytes()[i];
                if c == '.' as u8 {
                    line.replace_range(i..i + 1, "c");
                } else {
                    line.replace_range(i..i + 1, "b");
                }
            }
        }
    }
}

fn index(universe: &Universe) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for (row, line) in universe.galaxies.iter().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c as char == '#' {
                result.push((row as i64, col as i64));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_test() {
        let mut galaxies = Vec::new();
        galaxies.push("...#......".to_owned());
        galaxies.push(".......#..".to_owned());
        galaxies.push("#.........".to_owned());
        galaxies.push("..........".to_owned());
        galaxies.push("......#...".to_owned());
        galaxies.push(".#........".to_owned());
        galaxies.push(".........#".to_owned());
        galaxies.push("..........".to_owned());
        galaxies.push(".......#..".to_owned());
        galaxies.push("#...#.....".to_owned());
        let mut universe = Universe { galaxies };

        universe.expand();

        let mut expected = Vec::new();
        expected.push("..c#.c..c.".to_owned());
        expected.push("..c..c.#c.".to_owned());
        expected.push("#.c..c..c.".to_owned());
        expected.push("rrbrrbrrbr".to_owned());
        expected.push("..c..c#.c.".to_owned());
        expected.push(".#c..c..c.".to_owned());
        expected.push("..c..c..c#".to_owned());
        expected.push("rrbrrbrrbr".to_owned());
        expected.push("..c..c.#c.".to_owned());
        expected.push("#.c.#c..c.".to_owned());

        assert_eq!(expected, universe.galaxies);
    }

    #[test]
    fn solve_test() {
        let mut galaxies = Vec::new();
        galaxies.push("...#......".to_owned());
        galaxies.push(".......#..".to_owned());
        galaxies.push("#.........".to_owned());
        galaxies.push("..........".to_owned());
        galaxies.push("......#...".to_owned());
        galaxies.push(".#........".to_owned());
        galaxies.push(".........#".to_owned());
        galaxies.push("..........".to_owned());
        galaxies.push(".......#..".to_owned());
        galaxies.push("#...#.....".to_owned());
        let mut universe = Universe { galaxies };
        universe.expand();

        assert_eq!(374, solve(&universe, 2));
    }
}

fn solve(universe: &Universe, cost: i64) -> i64 {
    let galaxies = index(&universe);
    let mut answer = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let lhs = galaxies[i];
            let rhs = galaxies[j];
            let mut dist = 0;
            for row in i64::min(lhs.0, rhs.0)..i64::max(lhs.0, rhs.0) {
                let row = row as usize;
                let c = universe.galaxies[row].as_bytes()[0] as char;
                let step_cost = if c == 'b' || c == 'r' { cost } else { 1 };
                dist += step_cost;
            }
            for col in i64::min(lhs.1, rhs.1)..i64::max(lhs.1, rhs.1) {
                let col = col as usize;
                let c = universe.galaxies[0].as_bytes()[col] as char;
                let step_cost = if c == 'b' || c == 'c' { cost } else { 1 };
                dist += step_cost;
            }
            answer += dist;
        }
    }
    answer
}

pub mod part1 {
    use super::*;

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut universe = Universe::from_file(file_name);
            universe.expand();
            solve(&universe, 2).to_string()
        }

        fn day() -> i32 {
            11
        }

        fn part() -> i32 {
            1
        }
    }
}

pub mod part2 {
    use super::*;

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut universe = Universe::from_file(file_name);
            universe.expand();
            solve(&universe, 1000000).to_string()
        }

        fn day() -> i32 {
            11
        }

        fn part() -> i32 {
            2
        }
    }
}
