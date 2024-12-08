use std::collections::{HashMap, HashSet};

// cSpell: words antinodes drow dcol

type Point = aoc::Point<i32>;

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

fn antinodes_for_pair_of_points(p1: &Point, p2: &Point) -> (Point, Point) {
    let drow = p1.row - p2.row;
    let dcol = p1.col - p2.col;

    let mut res1 = p1.clone();
    let mut res2 = p2.clone();

    res1.row += drow;
    res1.col += dcol;

    res2.row -= drow;
    res2.col -= dcol;

    (res1, res2)
}

fn all_antinodes_for_pair_of_points(p1: &Point, p2: &Point, size: &Point) -> HashSet<Point> {
    let drow = p1.row - p2.row;
    let dcol = p1.col - p2.col;

    let mut result = HashSet::default();
    result.insert(p1.clone());
    result.insert(p2.clone());

    let mut p = p1.clone();
    loop {
        p.row += drow;
        p.col += dcol;
        if !is_inside(&p, size) {
            break;
        }
        result.insert(p.clone());
    }

    p = p2.clone();
    loop {
        p.row -= drow;
        p.col -= dcol;
        if !is_inside(&p, size) {
            break;
        }
        result.insert(p.clone());
    }

    result
}

fn is_inside(p: &Point, size: &Point) -> bool {
    p.row >= 0 && p.col >= 0 && p.row < size.row && p.col < size.col
}

fn get_antinode_positions(points: &AntennaPositions, size: &Point) -> HashSet<Point> {
    let mut result = HashSet::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (p1, p2) = antinodes_for_pair_of_points(&points[i], &points[j]);
            if is_inside(&p1, &size) {
                result.insert(p1);
            }
            if is_inside(&p2, &size) {
                result.insert(p2);
            }
        }
    }

    result
}

fn get_antinodes(points: &HashMap<char, AntennaPositions>, size: &Point) -> HashSet<Point> {
    let mut result = HashSet::new();

    for (_, positions) in points.iter() {
        let p = get_antinode_positions(positions, &size);
        result.extend(p.into_iter());
    }

    result
}

fn get_all_antinodes(points: &HashMap<char, AntennaPositions>, size: &Point) -> HashSet<Point> {
    let mut result = HashSet::new();

    for (_, positions) in points.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let antinodes = all_antinodes_for_pair_of_points(&positions[i], &positions[j], &size);
                result.extend(antinodes.into_iter());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day_08::get_all_antinodes;

    use super::{antinodes_for_pair_of_points, get_antinodes, parse_input, Point};

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

    #[test]
    fn antinodes_for_pair_of_points_test() {
        {
            let p1 = Point { row: 3, col: 2 };
            let p2 = Point { row: 4, col: 4 };

            let expected_points: HashSet<Point> = HashSet::from([Point { row: 2, col: 0 }, Point { row: 5, col: 6 }]);

            let (res1, res2) = antinodes_for_pair_of_points(&p1, &p2);
            assert_ne!(res1, res2);
            assert!(expected_points.contains(&res1));
            assert!(expected_points.contains(&res2));
        }

        {
            let p1 = Point { row: 3, col: 2 };
            let p2 = Point { row: 2, col: 4 };

            let expected_points: HashSet<Point> = HashSet::from([Point { row: 4, col: 0 }, Point { row: 1, col: 6 }]);

            let (res1, res2) = antinodes_for_pair_of_points(&p1, &p2);
            assert_ne!(res1, res2);
            assert!(expected_points.contains(&res1));
            assert!(expected_points.contains(&res2));
        }
    }

    #[test]
    fn get_antinodes_test() {
        {
            let lines = vec![
                "..........",
                "..........",
                "..........",
                "....a.....",
                "........a.",
                ".....a....",
                "..........",
                "..........",
                "..........",
                "..........",
            ];

            let (size, points) = parse_input(lines.iter());
            let antinodes = get_antinodes(&points, &size);
            assert_eq!(antinodes.len(), 4);
        }

        {
            let lines = vec![
                "............",
                "........0...",
                ".....0......",
                ".......0....",
                "....0.......",
                "......A.....",
                "............",
                "............",
                "........A...",
                ".........A..",
                "............",
                "............",
            ];
            let (size, points) = parse_input(lines.iter());
            let antinodes = get_antinodes(&points, &size);
            assert_eq!(antinodes.len(), 14);
        }
    }

    #[test]
    fn get_all_antinodes_test() {
        {
            let lines = vec![
                "T.........",
                "...T......",
                ".T........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
            ];

            let (size, points) = parse_input(lines.iter());
            let antinodes = get_all_antinodes(&points, &size);
            assert_eq!(antinodes.len(), 9);
        }
        {
            let lines = vec![
                "............",
                "........0...",
                ".....0......",
                ".......0....",
                "....0.......",
                "......A.....",
                "............",
                "............",
                "........A...",
                ".........A..",
                "............",
                "............",
            ];

            let (size, points) = parse_input(lines.iter());
            let antinodes = get_all_antinodes(&points, &size);
            assert_eq!(antinodes.len(), 34);
        }
    }
}

pub mod part1 {
    use super::{get_antinodes, parse_input};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (size, points) = parse_input(std::fs::read_to_string(input_file_name).unwrap().lines());
            let antinodes = get_antinodes(&points, &size);
            antinodes.len().to_string()
        }

        fn day() -> i32 {
            8
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2024
        }
    }
}

pub mod part2 {
    use super::{get_all_antinodes, parse_input};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (size, points) = parse_input(std::fs::read_to_string(input_file_name).unwrap().lines());
            let antinodes = get_all_antinodes(&points, &size);
            antinodes.len().to_string()
        }

        fn day() -> i32 {
            8
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }
}
