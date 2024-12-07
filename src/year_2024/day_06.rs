#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    #[default]
    Unknown = 0x0,
    North = 0x1,
    West = 0x2,
    South = 0x4,
    East = 0x8,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    pos: Point,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Empty,
    Obstacle,
}

#[derive(Default)]
struct Field {
    nodes: Vec<Vec<Node>>,
}

fn parse_field<I, S>(lines: I) -> (Field, Guard)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut field = Field::default();
    let mut guard = Guard::default();
    for (row_idx, line) in lines.enumerate() {
        let mut row = Vec::with_capacity(line.as_ref().len());
        for (col_idx, c) in line.as_ref().chars().enumerate() {
            match c {
                '.' => row.push(Node::Empty),
                '#' => row.push(Node::Obstacle),
                _ => {
                    guard.pos = Point {
                        row: row_idx as i32,
                        col: col_idx as i32,
                    };
                    match c {
                        '^' => guard.direction = Direction::North,
                        '>' => guard.direction = Direction::East,
                        'v' => guard.direction = Direction::South,
                        '<' => guard.direction = Direction::West,
                        _ => unreachable!(),
                    }
                }
            }
        }
        field.nodes.push(row);
    }
    (field, guard)
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
        use crate::day_06::{parse_field, Direction, Node, Point};

        #[test]
        fn test_parse_field() {
            let lines = vec![
                "....#.....",
                ".........#",
                "..........",
                "..#.......",
                ".......#..",
                "..........",
                ".#..^.....",
                "........#.",
                "#.........",
                "......#...",
            ];

            let (field, guard) = parse_field(lines.iter());
            assert_eq!(field.nodes.len(), 10);
            assert_eq!(field.nodes[0].len(), 10);

            assert_eq!(field.nodes[0][0], Node::Empty);
            assert_eq!(field.nodes[0][4], Node::Obstacle);
            assert_eq!(field.nodes[9][6], Node::Obstacle);

            assert_eq!(guard.pos, Point { row: 6, col: 4 });
            assert_eq!(guard.direction, Direction::North);
        }
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
