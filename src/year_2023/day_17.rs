use std::collections::{BinaryHeap, HashMap};

type Point = aoc::Point<i32>;

fn manhattan_distance(one: &Point, another: &Point) -> i32 {
    (one.row - another.row).abs() + (one.col - another.col).abs()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Unknown = 0,
    North = 0x1,
    West = 0x2,
    South = 0x4,
    East = 0x8,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Unknown => return Direction::Unknown,
            Direction::North => return Direction::South,
            Direction::West => return Direction::East,
            Direction::South => return Direction::North,
            Direction::East => return Direction::West,
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Waypoint {
    pos: Point,
    direction: Direction,
    heat_loss: i32,
    heuristic: i32,
    previous: Option<Point>,
}

impl Ord for Waypoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.heat_loss + other.heuristic).cmp(&(self.heat_loss + &self.heuristic))
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

fn get_straight_path_length(direction: Direction, point: Point, waypoints: &Vec<Vec<Option<Waypoint>>>) -> i32 {
    let mut result = 0;
    let mut point = point;
    loop {
        let waypoint = waypoints[point.row as usize][point.col as usize];
        match waypoint {
            None => break,
            Some(waypoint) => {
                if waypoint.direction != direction {
                    break;
                }
                result += 1;
                match waypoint.previous {
                    None => break,
                    Some(new_point) => point = new_point,
                }
            }
        }
    }
    result
}

fn get_empty_waypoints(field: &Field) -> Vec<Vec<Option<Waypoint>>> {
    let mut result: Vec<Vec<Option<Waypoint>>> = Vec::with_capacity(field.nodes.len());
    for row in field.nodes.iter() {
        result.push(vec![None; row.len()]);
    }
    result
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

    fn traverse(&self, start: Point, finish: Point) -> Option<i32> {
        let mut heap = BinaryHeap::new();

        let mut visited = get_empty_waypoints(&self);

        let start_waypoint = Waypoint {
            pos: start,
            direction: Direction::Unknown,
            heat_loss: 0,
            heuristic: manhattan_distance(&start, &finish),
            previous: None,
        };

        heap.push(start_waypoint);

        while let Some(waypoint) = heap.pop() {
            if waypoint.pos == finish {
                return Some(waypoint.heat_loss);
            }

            let mut neighbors = self.get_all_neighbors(waypoint.pos);

            // ignore backward path
            neighbors.remove(&waypoint.direction.opposite());

            for (direction, point) in neighbors.iter() {
                let heat_loss = waypoint.heat_loss + self.nodes[point.row as usize][point.col as usize];

                // ignore path if it's too long
                if let Some(prev_point) = waypoint.previous {
                    if get_straight_path_length(*direction, prev_point, &visited) > 2 {
                        continue;
                    }
                }

                let new_waypoint = Waypoint {
                    pos: *point,
                    direction: *direction,
                    heat_loss: heat_loss,
                    heuristic: manhattan_distance(&point, &finish),
                    previous: Some(waypoint.pos),
                };

                // update already seen paths
                if let Some(visited_waypoint) = visited[point.row as usize][point.col as usize] {
                    if visited_waypoint.heat_loss + visited_waypoint.heuristic
                        < new_waypoint.heat_loss + new_waypoint.heuristic
                    {
                        continue;
                    }
                }

                heap.push(new_waypoint);
            }
            visited[waypoint.pos.row as usize][waypoint.pos.col as usize] = Some(waypoint);
        }

        None
    }
}

pub mod part1 {
    use crate::day_17::{Field, Point};
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let field = Field::from_file(file_name);

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

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_17::{get_empty_waypoints, get_straight_path_length, Direction, Field, Point, Waypoint};

        #[test]
        fn cmp_test() {
            let w1 = Waypoint {
                pos: Point { row: 0, col: 0 },
                direction: Direction::Unknown,
                heat_loss: 1,
                heuristic: 5,
                previous: None,
            };

            let w2 = Waypoint {
                pos: Point { row: 0, col: 0 },
                direction: Direction::Unknown,
                heat_loss: 12,
                heuristic: 5,
                previous: None,
            };

            assert!(w1 > w2);
            assert!(w2 < w1);
            assert!(w1 == w1);
            assert!(w2 == w2);

            let w3 = Waypoint {
                pos: Point { row: 0, col: 0 },
                direction: Direction::Unknown,
                heat_loss: 1,
                heuristic: 150,
                previous: None,
            };

            assert!(w1 > w3);
            assert!(w3 < w1);
            assert!(w3 == w3);
        }

        #[test]
        fn test_get_empty_waypoints() {
            let lines = vec!["123", "456", "019"];
            let field = Field::from_lines(&lines);
            let waypoints = get_empty_waypoints(&field);

            for (f_row, w_row) in std::iter::zip(field.nodes.iter(), waypoints.iter()) {
                assert_eq!(f_row.len(), w_row.len());
                for w in w_row {
                    assert!(w.is_none());
                }
            }
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
        fn get_straight_path_length_test() {}

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

            let field = Field::from_lines(&lines);

            assert_eq!(field.col_count, 13);
            assert_eq!(field.row_count, 13);

            let start_point = Point { row: 0, col: 0 };
            let finish_point = Point { row: 12, col: 12 };

            let result = field.traverse(start_point, finish_point);
            assert_eq!(result, Some(102));
        }
    }
}
