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
        let empty_line = ".".repeat(self.galaxies[0].len()).to_owned();
        let mut indicies = Vec::new();
        for (i, line) in self.galaxies.iter().enumerate() {
            if line.as_bytes().iter().all(|c| *c == '.' as u8) {
                indicies.push(i);
            }
        }
        for i in indicies.iter().rev() {
            self.galaxies.insert(*i, empty_line.clone());
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
            for line in self.galaxies.iter_mut() {
                line.insert(*i, '.');
            }
        }
    }
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
        expected.push("....#........".to_owned());
        expected.push(".........#...".to_owned());
        expected.push("#............".to_owned());
        expected.push(".............".to_owned());
        expected.push(".............".to_owned());
        expected.push("........#....".to_owned());
        expected.push(".#...........".to_owned());
        expected.push("............#".to_owned());
        expected.push(".............".to_owned());
        expected.push(".............".to_owned());
        expected.push(".........#...".to_owned());
        expected.push("#....#.......".to_owned());

        assert_eq!(expected, universe.galaxies);
    }
}

pub mod part1 {
    use super::*;

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

    fn solve(universe: &Universe) -> i64 {
        let galaxies = index(&universe);
        let mut answer = 0;
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                let lhs = galaxies[i];
                let rhs = galaxies[j];
                let dist = (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs();
                answer += dist;
            }
        }
        answer
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut universe = Universe::from_file(file_name);
            universe.expand();
            solve(&universe).to_string()
        }

        fn day() -> i32 {
            11
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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

            assert_eq!(374, solve(&universe));
        }
    }
}
