use aoc::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Node {
    #[default]
    Empty,
    Roll,
}

type Field = aoc::Field<Node>;

fn parse_field<I, S>(lines: I) -> Option<Field>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut cols: Option<usize> = None;
    let mut nodes = Vec::default();
    for line in lines {
        let line_len = line.as_ref().len();
        match cols {
            Some(count) => {
                if count != line_len {
                    return None;
                }
            }
            None => cols = Some(line_len),
        }

        for c in line.as_ref().chars() {
            match c {
                '.' => nodes.push(Node::Empty),
                '@' => nodes.push(Node::Roll),
                _ => unreachable!(),
            }
        }
    }

    Field::from_flat_vector(nodes, cols.expect("Col count not found"))
}

fn count_neighbors(field: &Field, point: Point<i32>) -> i32 {
    let mut result = 0;
    for n in point.neighbors_all() {
        if !field.is_inside(n.row, n.col) {
            continue;
        }
        if *field.get(n.row as usize, n.col as usize) == Node::Roll {
            result += 1;
        }
    }
    result
}

pub mod part1 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use aoc::Point;

    use crate::day_04::{count_neighbors, parse_field, Field, Node};

    fn solve(field: &Field) -> i32 {
        let mut result = 0;

        for col in 0..field.get_col_count() as i32 {
            for row in 0..field.get_row_count() as i32 {
                if *field.get(row as usize, col as usize) == Node::Empty {
                    continue;
                }
                let point = Point { row: row, col: col };
                if count_neighbors(&field, point) < 4 {
                    result += 1;
                }
            }
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let file = File::open(file_name).unwrap();
            let reader = BufReader::new(file);
            let field =
                parse_field(std::fs::read_to_string(file_name).unwrap().lines()).expect("Unable to parse field");
            solve(&field).to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            4
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_04::{parse_field, part1::solve};

        #[test]
        fn test() {
            let input = vec![
                "..........",
                "..@@.@@@@.",
                "@@@.@.@.@@",
                "@@@@@.@.@@",
                "@.@@@@..@.",
                "@@.@@@@.@@",
                ".@@@@@@@.@",
                ".@.@.@.@@@",
                "@.@@@.@@@@",
                ".@@@@@@@@.",
                "@.@.@@@.@.",
                "..........",
            ];
            let field = parse_field(input.iter());
            assert!(field.is_some());
            let field = field.unwrap();

            assert_eq!(field.get_row_count(), 12);
            assert_eq!(field.get_col_count(), 10);

            let result = solve(&field);
            assert_eq!(result, 13);
        }
    }
}

pub mod part2 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use aoc::Point;

    use crate::day_04::{count_neighbors, parse_field, Field, Node};

    struct MoveResult {
        field: Field,
        moved: i32,
    }

    fn move_rolls(field: &Field) -> MoveResult {
        let mut result = MoveResult {
            field: field.clone(),
            moved: 0,
        };

        for col in 0..field.get_col_count() as i32 {
            for row in 0..field.get_row_count() as i32 {
                if *field.get(row as usize, col as usize) == Node::Empty {
                    continue;
                }
                let point = Point { row: row, col: col };
                if count_neighbors(&field, point) < 4 {
                    *result.field.get_mut(row as usize, col as usize) = Node::Empty;
                    result.moved += 1;
                }
            }
        }

        result
    }

    fn solve(field: &Field) -> i32 {
        let mut result = 0;

        let mut current_field = field.clone();
        loop {
            let move_result = move_rolls(&current_field);
            if move_result.moved == 0 {
                break;
            }
            result += move_result.moved;
            current_field = move_result.field;
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let file = File::open(file_name).unwrap();
            let reader = BufReader::new(file);
            let field =
                parse_field(std::fs::read_to_string(file_name).unwrap().lines()).expect("Unable to parse field");
            solve(&field).to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            4
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_04::{parse_field, part2::solve};

        #[test]
        fn test() {
            let input = vec![
                "..........",
                "..@@.@@@@.",
                "@@@.@.@.@@",
                "@@@@@.@.@@",
                "@.@@@@..@.",
                "@@.@@@@.@@",
                ".@@@@@@@.@",
                ".@.@.@.@@@",
                "@.@@@.@@@@",
                ".@@@@@@@@.",
                "@.@.@@@.@.",
                "..........",
            ];
            let field = parse_field(input.iter());
            assert!(field.is_some());
            let field = field.unwrap();

            assert_eq!(field.get_row_count(), 12);
            assert_eq!(field.get_col_count(), 10);

            let result = solve(&field);
            assert_eq!(result, 43);
        }
    }
}
