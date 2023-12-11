#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: u64,
    col: u64,
}

#[derive(Default)]
struct Map {
    nodes: Vec<String>,
    start: Point,
}

impl Map {
    fn from_file(file_name: &str) -> Map {
        let mut nodes: Vec<String> = Vec::new();
        let mut start = Point::default();
        let mut row = 0;
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            if let Some(column) = line.as_bytes().iter().position(|x| *x == 'S' as u8) {
                start.row = row;
                start.col = column as u64;
            }
            nodes.push(line.to_owned());
            row += 1;
        }
        Map { nodes: nodes, start }
    }

    fn get_possible_paths(self: &Map, p: &Point) -> (Point, Point) {
        let c = self.nodes[p.row as usize].as_bytes()[p.col as usize] as char;
        match c {
            '|' => (
                Point {
                    row: p.row - 1,
                    col: p.col,
                },
                Point {
                    row: p.row + 1,
                    col: p.col,
                },
            ),
            '-' => (
                Point {
                    row: p.row,
                    col: p.col - 1,
                },
                Point {
                    row: p.row,
                    col: p.col + 1,
                },
            ),
            'L' => (
                Point {
                    row: p.row - 1,
                    col: p.col,
                },
                Point {
                    row: p.row,
                    col: p.col + 1,
                },
            ),
            'J' => (
                Point {
                    row: p.row - 1,
                    col: p.col,
                },
                Point {
                    row: p.row,
                    col: p.col - 1,
                },
            ),
            '7' => (
                Point {
                    row: p.row,
                    col: p.col - 1,
                },
                Point {
                    row: p.row + 1,
                    col: p.col,
                },
            ),
            'F' => (
                Point {
                    row: p.row,
                    col: p.col + 1,
                },
                Point {
                    row: p.row + 1,
                    col: p.col,
                },
            ),
            'S' => self.get_possible_paths_from_start(),
            _ => unreachable!(),
        }
    }

    fn get_possible_paths_from_start(self: &Map) -> (Point, Point) {
        let indicies = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];
        let mut result = Vec::new();
        for i in indicies.iter() {
            let r = self.start.row as i32 + i.0;
            let c = self.start.col as i32 + i.1;
            if r < 0 {
                continue;
            }
            if c < 0 {
                continue;
            }
            if r >= self.nodes.len() as i32 {
                continue;
            }
            if c >= self.nodes[self.start.row as usize].as_bytes().len() as i32 {
                continue;
            }
            let value = self.nodes[r as usize].as_bytes()[c as usize] as char;
            if value == '.' {
                continue;
            }

            let p = Point {
                row: r as u64,
                col: c as u64,
            };
            let (p1, p2) = self.get_possible_paths(&p);
            if p1 == self.start {
                result.push(p);
            }
            if p2 == self.start {
                result.push(p);
            }
        }
        assert_eq!(2, result.len());
        (result[0], result[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_possible_paths_test() {
        let mut map = Map::default();
        map.nodes.push("7-F7-".to_owned());
        map.nodes.push(".FJ|7".to_owned());
        map.nodes.push("SJLL7".to_owned());
        map.nodes.push("|F--J".to_owned());
        map.nodes.push("LJ.LJ".to_owned());
        map.start = Point { row: 2, col: 0 };

        let (answer1, answer2) = map.get_possible_paths(&Point { row: 3, col: 1 });
        assert_eq!(Point { row: 3, col: 2 }, answer1);
        assert_eq!(Point { row: 4, col: 1 }, answer2);

        let (answer1, answer2) = map.get_possible_paths(&Point { row: 0, col: 2 });
        assert_eq!(Point { row: 0, col: 3 }, answer1);
        assert_eq!(Point { row: 1, col: 2 }, answer2);

        let (answer1, answer2) = map.get_possible_paths_from_start();
        assert_eq!(Point { row: 3, col: 0 }, answer1);
        assert_eq!(Point { row: 2, col: 1 }, answer2);
    }
}

fn traverse(map: &Map) -> std::collections::HashSet<Point> {
    let mut visited = std::collections::HashSet::new();

    let (mut p, _) = map.get_possible_paths_from_start();
    visited.insert(map.start);
    visited.insert(p);

    loop {
        let (p1, p2) = map.get_possible_paths(&p);
        if visited.contains(&p1) && visited.contains(&p2) {
            break;
        }

        if visited.contains(&p1) {
            visited.insert(p2.clone());
            p = p2;
        } else if visited.contains(&p2) {
            visited.insert(p1.clone());
            p = p1;
        }
    }
    visited
}

pub mod part1 {
    use super::*;

    fn solve(map: &Map) -> u64 {
        assert!(!map.nodes.is_empty());
        let visited = traverse(map);
        visited.len() as u64 / 2
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let map = Map::from_file(file_name);
            solve(&map).to_string()
        }

        fn day() -> i32 {
            10
        }

        fn part() -> i32 {
            1
        }
    }

    #[test]
    fn solve_test() {
        let mut map = Map::default();
        map.nodes.push("..F7.".to_owned());
        map.nodes.push(".FJ|.".to_owned());
        map.nodes.push("SJ.L7".to_owned());
        map.nodes.push("|F--J".to_owned());
        map.nodes.push("LJ...".to_owned());
        map.start = Point { row: 2, col: 0 };

        assert_eq!(8, solve(&map));
    }
}
