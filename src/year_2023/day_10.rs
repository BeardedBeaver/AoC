type Point = aoc::Point<u64>;

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
        let indices = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];
        let mut result = Vec::new();
        for i in indices.iter() {
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

    fn get_start_pipe(self: &Map) -> char {
        let (p1, p2) = self.get_possible_paths_from_start();
        let r1 = p1.row as i64 - self.start.row as i64;
        let c1 = p1.col as i64 - self.start.col as i64;
        let r2 = p2.row as i64 - self.start.row as i64;
        let c2 = p2.col as i64 - self.start.col as i64;

        match (r1, c1, r2, c2) {
            (0, -1, 0, 1) => '-',
            (-1, 0, 1, 0) => '|',
            (1, 0, 0, 1) => 'F',
            (0, -1, 1, 0) => '7',
            (0, -1, -1, 0) => 'J',
            (-1, 0, 0, 1) => 'L',
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)] // used in tests
    fn assign(self: &mut Map, row: usize, col: usize, c: char) {
        self.nodes[row].replace_range(col..col + 1, &c.to_string());
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

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
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

        fn year() -> i32 {
            2023
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

pub mod part2 {
    use super::*;

    fn is_inside(intersections: &Vec<char>) -> bool {
        let mut s: String = intersections.into_iter().collect();
        s = s.replace("LJ", "");
        s = s.replace("F7", "");

        s = s.replace("L7", "|");
        s = s.replace("FJ", "|");
        s.len() % 2 == 1
    }

    // Here we go over all Points, skip ones that are the loop itself, and for each Point
    // count how many "edges" of the loop we cross going from this node to the right.
    // The trick is to correctly handle going by the edge (see is_inside function for this).
    // Note that we need to count not only blank Points, but pipes that are not the part
    // of the main loop as well.
    fn solve(map: &Map) -> u64 {
        assert!(!map.nodes.is_empty());
        let visited = traverse(map);
        let mut result = 0;
        let accepted_pipes = vec!['|', 'L', 'J', '7', 'F', 'S'];
        for (row, line) in map.nodes.iter().enumerate() {
            for (col, _) in line.as_bytes().iter().enumerate() {
                if visited.contains(&Point {
                    row: row as u64,
                    col: col as u64,
                }) {
                    continue;
                }
                let mut intersections = Vec::new();
                for c in col..line.len() {
                    let mut value = line.as_bytes()[c] as char;
                    if visited.contains(&Point {
                        row: row as u64,
                        col: c as u64,
                    }) && accepted_pipes.contains(&value)
                    {
                        if value == 'S' {
                            value = map.get_start_pipe();
                        }
                        intersections.push(value);
                    }
                }
                if is_inside(&intersections) {
                    result += 1;
                }
            }
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let map = Map::from_file(file_name);
            solve(&map).to_string()
        }

        fn day() -> i32 {
            10
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }

    #[test]
    fn solve_test() {
        let mut map = Map::default();
        map.nodes.push("...........".to_owned());
        map.nodes.push(".S-------7.".to_owned());
        map.nodes.push(".|F-----7|.".to_owned());
        map.nodes.push(".||.....||.".to_owned());
        map.nodes.push(".||.....||.".to_owned());
        map.nodes.push(".|L-7.F-J|.".to_owned());
        map.nodes.push(".|..|.|..|.".to_owned());
        map.nodes.push(".L--J.L--J.".to_owned());
        map.nodes.push("...........".to_owned());
        map.start = Point { row: 1, col: 1 };

        assert_eq!(4, solve(&map));

        let mut map = Map::default();
        map.nodes.push(".F----7F7F7F7F-7....".to_owned());
        map.nodes.push(".|F--7||||||||FJ....".to_owned());
        map.nodes.push(".||.FJ||||||||L7....".to_owned());
        map.nodes.push("FJL7L7LJLJ||LJ.L-7..".to_owned());
        map.nodes.push("L--J.L7...LJS7F-7L7.".to_owned());
        map.nodes.push("....F-J..F7FJ|L7L7L7".to_owned());
        map.nodes.push("....L7.F7||L7|.L7L7|".to_owned());
        map.nodes.push(".....|FJLJ|FJ|F7|.LJ".to_owned());
        map.nodes.push("....FJL-7.||.||||...".to_owned());
        map.nodes.push("....L---J.LJ.LJLJ...".to_owned());
        map.start = Point { row: 4, col: 12 };

        assert_eq!(
            map.nodes[map.start.row as usize].as_bytes()[map.start.col as usize],
            'S' as u8
        );

        assert_eq!(8, solve(&map));
    }

    #[test]
    fn is_inside_test() {
        assert_eq!(true, is_inside(&vec!['|']));
        assert_eq!(false, is_inside(&vec!['|', '|']));
        assert_eq!(false, is_inside(&vec!['F', '7', '|', '|']));
        assert_eq!(false, is_inside(&vec!['F', 'J', '|']));
    }

    #[test]
    fn get_start_pipe_test() {
        use crate::day_10::Point;
        let mut map = Map::default();

        let original_map = vec![
            ".....".to_owned(),
            ".F-7.".to_owned(),
            ".|.|.".to_owned(),
            ".L-J.".to_owned(),
            ".....".to_owned(),
        ];

        for (row, line) in original_map.iter().enumerate() {
            for (col, tile) in line.chars().enumerate() {
                if tile != '.' {
                    map.nodes = original_map.clone();
                    map.start = Point {
                        row: row as u64,
                        col: col as u64,
                    };
                    map.assign(row, col, 'S');
                    assert_eq!(tile, map.get_start_pipe());
                }
            }
        }
    }
}
