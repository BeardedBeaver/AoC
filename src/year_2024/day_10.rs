use aoc::Point;

#[derive(Default, Debug, Clone)]
struct Field {
    nodes: Vec<Vec<i8>>,
}

fn parse_field<I, S>(lines: I) -> Field
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut field = Field::default();

    for line in lines {
        let mut row = Vec::with_capacity(line.as_ref().len());
        for c in line.as_ref().chars() {
            row.push(c.to_digit(10).unwrap() as i8);
        }
        field.nodes.push(row);
    }

    field
}

// How many 9's we can approach from the specified 0
fn get_hiking_score(field: &Field, start: &Point<i32>) -> i32 {
    0
}

fn get_hiking_paths_score(field: &Field) -> i32 {
    let mut result = 0;
    for (row_idx, row) in field.nodes.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value != 0 {
                continue;
            }
            result += get_hiking_score(
                field,
                &Point {
                    row: row_idx as i32,
                    col: col_idx as i32,
                },
            );
        }
    }

    result
}

pub mod part1 {
    use super::{get_hiking_paths_score, parse_field};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let field = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            get_hiking_paths_score(&field).to_string()
        }

        fn day() -> i32 {
            10
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
            10
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
        #[test]
        fn test() {}
    }
}
