use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Unknown = 0,
    North = 0x1,
    West = 0x2,
    South = 0x4,
    East = 0x8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Waypoint {
    pos: Point,
    direction: Direction,
    heat_loss: i32,
    previous: Option<Point>,
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
    nodes: Vec<Vec<i32>>,
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
                line.push(c.to_digit(10).unwrap() as i32);
            }
            result.nodes.push(line);
        }
        result.row_count = result.nodes.len() as i32;
        result
    }

    fn get_all_neighbors(&self, point: Point) -> HashMap<Direction, Point> {
        let neighbors = HashMap::from([
            (
                Direction::North,
                Point {
                    row: point.row - 1,
                    col: point.col,
                },
            ),
            (
                Direction::South,
                Point {
                    row: point.row + 1,
                    col: point.col,
                },
            ),
            (
                Direction::West,
                Point {
                    row: point.row,
                    col: point.col - 1,
                },
            ),
            (
                Direction::East,
                Point {
                    row: point.row,
                    col: point.col + 1,
                },
            ),
        ]);

        let mut result = HashMap::new();

        for (direction, point) in neighbors.into_iter() {
            if point.col < 0 || point.row < 0 || point.col >= self.col_count || point.row >= self.row_count {
                continue;
            }
            result.insert(direction, point);
        }

        result
    }

    fn traverse(&mut self, start: Point, finish: Point) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        let mut waypoints: HashMap<Point, Waypoint> = HashMap::new();

        let start_waypoint = Waypoint {
            pos: start,
            direction: Direction::Unknown,
            heat_loss: 0,
            previous: None,
        };

        waypoints.insert(start, start_waypoint);
        heap.push(start_waypoint);

        while let Some(waypoint) = heap.pop() {
            if waypoint.pos.col == finish.col && waypoint.pos.row == finish.row {
                return Some(waypoint.heat_loss);
            }
            let mut neighbors = self.get_all_neighbors(waypoint.pos);
            for (direction, point) in neighbors.into_iter() {
                let waypoint = Waypoint {
                    pos: point,
                    direction: direction,
                    heat_loss: waypoint.heat_loss + self.nodes[point.row as usize][point.col as usize],
                    previous: Some(point),
                };
                if let Some(existing_waypoint) = waypoints.get(&point) {
                    if existing_waypoint.heat_loss >= waypoint.heat_loss {
                        continue;
                    }
                }
                waypoints.insert(point, waypoint);
                heap.push(waypoint);
            }
        }

        None
    }
}

pub mod part1 {
    use crate::day_17::{Field, Point};
    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut field = Field::from_file(file_name);

            let start_point = Point { row: 0, col: 0 };
            let finish_point = Point {
                row: field.row_count - 1,
                col: field.col_count - 1,
            };

            let score = field.traverse(start_point, finish_point).expect("path not found");
            score.to_string()
        }

        fn day() -> i32 {
            17
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_17::{Direction, Field, Point, Waypoint};

        #[test]
        fn cmp_test() {
            let w1 = Waypoint {
                pos: Point { row: 0, col: 0 },
                direction: Direction::Unknown,
                heat_loss: 1,
                previous: None,
            };

            let w2 = Waypoint {
                pos: Point { row: 0, col: 0 },
                direction: Direction::Unknown,
                heat_loss: 12,
                previous: None,
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

            let start_point = Point { row: 0, col: 0 };
            let finish_point = Point { row: 12, col: 12 };

            let result = field.traverse(start_point, finish_point);

            assert_eq!(result, Some(102));
        }
    }
}
