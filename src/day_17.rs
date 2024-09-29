use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Unknown = 0,
    North = 0x1,
    West = 0x2,
    South = 0x4,
    East = 0x8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Waypoint {
    row: usize,
    col: usize,
    prev_direction: Direction,
    straight_nodes: i32,
    heat_loss: usize,
}

impl Ord for Waypoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Waypoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Default, Clone)]
struct Field {
    nodes: Vec<Vec<u32>>,
    row_count: i32,
    col_count: i32,
}

impl Field {
    fn from_file(file_name: &str) -> Field {
        let file_content = std::fs::read_to_string(file_name).unwrap();
        let lines = file_content.lines().collect::<Vec<_>>();
        Field::from_lines(&lines)
    }

    fn from_lines(lines: &Vec<&str>) -> Field {
        let mut result = Field::default();
        for s in lines.iter() {
            let mut line = Vec::with_capacity(s.len());
            result.col_count = s.len() as i32;
            for c in s.chars() {
                line.push(c.to_digit(10).unwrap());
            }
            result.nodes.push(line);
        }
        result.row_count = result.nodes.len() as i32;
        result
    }

    fn traverse(&mut self, start_row: usize, start_col: usize, finish_row: usize, finish_col: usize) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        heap.push(Waypoint {
            row: start_row,
            col: start_col,
            prev_direction: Direction::Unknown,
            straight_nodes: 0,
            heat_loss: 0,
        });

        while let Some(waypoint) = heap.pop() {
            if waypoint.col == finish_col && waypoint.row == finish_row {
                return Some(waypoint.heat_loss);
            }
        }

        None
    }
}

pub mod part1 {
    #[cfg(test)]
    mod tests {
        use std::vec;

        use crate::day_17::{Direction, Field, Waypoint};

        use super::*;

        #[test]
        fn cmp_test() {
            let w1 = Waypoint {
                row: 0,
                col: 0,
                prev_direction: Direction::Unknown,
                straight_nodes: 0,
                heat_loss: 1,
            };

            let w2 = Waypoint {
                row: 0,
                col: 0,
                prev_direction: Direction::Unknown,
                straight_nodes: 0,
                heat_loss: 12,
            };

            assert!(w1 > w2);
            assert!(w2 < w1);
            assert!(w1 == w1);
            assert!(w2 == w2);
        }

        #[test]
        fn from_lines_test() {
            let lines = vec!["123", "456", "019"];
            let field = Field::from_lines(&lines);

            assert_eq!(field.nodes[0][0], 1);
            assert_eq!(field.nodes[0][1], 2);
            assert_eq!(field.nodes[0][2], 3);

            assert_eq!(field.nodes[1][0], 4);
            assert_eq!(field.nodes[1][1], 5);
            assert_eq!(field.nodes[1][2], 6);

            assert_eq!(field.nodes[2][0], 0);
            assert_eq!(field.nodes[2][1], 1);
            assert_eq!(field.nodes[2][2], 9);
        }

        #[test]
        fn traverse_test() {
            let lines = vec![
                "2413432311323",
                "3215453535623",
                "3255245654254",
                "3446585845452",
                "4546657867536",
                "1438598798454",
                "4457876987766",
                "3637877979653",
                "4654967986887",
                "4564679986453",
                "1224686865563",
                "2546548887735",
                "4322674655533",
            ];

            let mut field = Field::from_lines(&lines);

            assert_eq!(field.col_count, 13);
            assert_eq!(field.row_count, 13);

            let result = field.traverse(0, 0, 12, 12);

            assert_eq!(result, Some(102));
        }
    }
}
