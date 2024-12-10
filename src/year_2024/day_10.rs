use std::collections::{HashSet, VecDeque};

type Point = aoc::Point<i32>;

#[derive(Default, Debug, Clone)]
struct Field {
    nodes: Vec<Vec<i32>>,
}

fn get_neighbors(point: &Point) -> Vec<Point> {
    vec![
        Point {
            row: point.row + 1,
            col: point.col,
        },
        Point {
            row: point.row - 1,
            col: point.col,
        },
        Point {
            row: point.row,
            col: point.col + 1,
        },
        Point {
            row: point.row,
            col: point.col - 1,
        },
    ]
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
            row.push(c.to_digit(10).unwrap() as i32);
        }
        field.nodes.push(row);
    }

    field
}

// How many 9's we can approach from the specified 0
fn get_hiking_score(field: &Field, start: &Point, skip_visited: bool) -> i32 {
    let mut result = 0;

    let mut queue: VecDeque<Point> = VecDeque::default();
    queue.push_back(Point {
        row: start.row,
        col: start.col,
    });

    let mut visited: HashSet<Point> = HashSet::default();
    visited.insert(Point {
        row: start.row,
        col: start.col,
    });

    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        let height = field.nodes[point.row as usize][point.col as usize];

        if height == 9 {
            result += 1;
            continue;
        }

        let neighbors = get_neighbors(&point);
        for n in neighbors.into_iter() {
            if n.row < 0 || n.col < 0 || n.row >= field.nodes.len() as i32 || n.col >= field.nodes[0].len() as i32 {
                continue;
            }

            if field.nodes[n.row as usize][n.col as usize] != height + 1 {
                continue;
            }

            if skip_visited {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n.clone());
            }

            queue.push_back(n);
        }
    }

    result
}

fn get_hiking_paths_score(field: &Field, skip_visited: bool) -> i32 {
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
                skip_visited,
            );
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day_10::get_hiking_paths_score;

    use super::parse_field;

    #[test]
    fn parse_field_test() {
        let lines = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801", "10456732",
        ];
        let field = parse_field(lines.iter());
        assert_eq!(field.nodes.len(), 8);
        assert_eq!(field.nodes[0].len(), 8);

        assert_eq!(field.nodes[0][0], 8);
        assert_eq!(field.nodes[1][1], 8);
        assert_eq!(field.nodes[2][2], 4);
        assert_eq!(field.nodes[3][3], 4);
        assert_eq!(field.nodes[4][4], 8);
        assert_eq!(field.nodes[5][5], 0);
        assert_eq!(field.nodes[6][6], 0);
        assert_eq!(field.nodes[7][7], 2);
    }

    #[test]
    fn get_hiking_paths_score_test() {
        let lines = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801", "10456732",
        ];
        let field = parse_field(lines.iter());
        assert_eq!(get_hiking_paths_score(&field, true), 36);
        assert_eq!(get_hiking_paths_score(&field, false), 81);
    }
}

pub mod part1 {
    use super::{get_hiking_paths_score, parse_field};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let field = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            get_hiking_paths_score(&field, true).to_string()
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
    use super::{get_hiking_paths_score, parse_field};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let field = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            get_hiking_paths_score(&field, false).to_string()
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
