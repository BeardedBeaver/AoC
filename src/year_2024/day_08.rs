use std::collections::HashMap;

#[derive(Debug, Default)]
struct Point {
    row: i32,
    col: i32,
}

type AntennaPositions = Vec<Point>;

fn parse_input<I, S>(lines: I) -> (Point, HashMap<char, AntennaPositions>)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = HashMap::new();
    let mut size = Point::default();
    for (row_idx, line) in lines.enumerate() {
        // update size row
        if row_idx as i32 > size.row {
            size.row = row_idx as i32;
        }
        for (col_idx, c) in line.as_ref().chars().enumerate() {
            // update size col
            if col_idx as i32 > size.col {
                size.col = col_idx as i32;
            }
            if c == '.' {
                continue;
            }
            let entry = result.entry(c).or_insert(Vec::new());
            entry.push(Point {
                row: row_idx as i32,
                col: col_idx as i32,
            });
        }
    }
    size.row += 1;
    size.col += 1;
    (size, result)
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    #[test]
    fn parse_input_basic_test() {
        let lines = vec!["....", ".aa.", "...b"];
        let (size, points) = parse_input(lines.iter());
        assert_eq!(size.row, 3);
        assert_eq!(size.col, 4);

        assert!(points.contains_key(&'a'));
        let a = points.get(&'a').unwrap();
        assert_eq!(a.len(), 2);
        assert_eq!(a[0].row, 1);
        assert_eq!(a[0].col, 1);
        assert_eq!(a[1].row, 1);
        assert_eq!(a[1].col, 2);

        assert!(points.contains_key(&'b'));
        let b = points.get(&'b').unwrap();
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].row, 2);
        assert_eq!(b[0].col, 3);
    }
}

pub mod part1 {
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
