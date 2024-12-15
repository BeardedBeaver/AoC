use std::collections::VecDeque;

type Point = aoc::Point<i32>;
type Field = aoc::Field<u8>;

fn parse_field<I, S>(lines: I) -> Field
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut nodes = Vec::new();
    let mut col_count: Option<usize> = None;
    for line in lines {
        if col_count == None {
            col_count = Some(line.as_ref().len());
        }
        for c in line.as_ref().chars() {
            nodes.push(c as u8);
        }
    }

    Field::from_flat_vector(nodes, col_count.unwrap()).unwrap()
}

struct GardenParams {
    perimeter: i32,
    area: i32,
    sides_count: i32,
}

fn compute_perimeter(field: &Field, points: &Vec<Point>) -> i32 {
    assert!(!points.is_empty());
    let mut result = 0;
    let value = field.get(points[0].row as usize, points[0].col as usize).clone();
    for point in points.iter() {
        for neighbor in point.neighbors_orthogonal() {
            if let Some(nv) = field.try_get(neighbor.row as usize, neighbor.col as usize) {
                if value != *nv {
                    result += 1;
                }
            } else {
                result += 1;
            }
        }
    }

    result
}

fn get_garden_params(field: &mut Field, origin: &Point) -> GardenParams {
    let mut nodes = Vec::new();
    let mut queue = VecDeque::new();

    let row_count = field.get_row_count() as i32;
    let col_count = field.get_col_count() as i32;

    queue.push_back(origin.clone());
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();

        visit_node(field, &point);
        for n in point.neighbors_orthogonal() {
            if n.row < 0 || n.col < 0 || n.row >= row_count || n.col >= col_count {
                continue;
            }
            if is_visited(field, &n) {
                continue;
            }
            if equal(
                *field.get(point.row as usize, point.col as usize),
                *field.get(n.row as usize, n.col as usize),
            ) {
                visit_node(field, &n);
                queue.push_back(n);
            }
        }

        nodes.push(point);
    }
    GardenParams {
        perimeter: compute_perimeter(&field, &nodes),
        area: nodes.len() as i32,
        sides_count: 0,
    }
}

fn equal(lhs: u8, rhs: u8) -> bool {
    lhs & 127 == rhs & 127
}

fn is_visited(field: &Field, pos: &Point) -> bool {
    is_node_visited(*field.get(pos.row as usize, pos.col as usize))
}

fn is_node_visited(node: u8) -> bool {
    node & 128 != 0
}

fn visit(node: &mut u8) {
    *node |= 128;
}

fn visit_node(field: &mut Field, pos: &Point) {
    visit(&mut field.get_mut(pos.row as usize, pos.col as usize));
}

trait Calculator {
    fn calc(params: &GardenParams) -> i32;
}

struct CalculatorPart1 {}

impl Calculator for CalculatorPart1 {
    fn calc(params: &GardenParams) -> i32 {
        params.area * params.perimeter
    }
}

struct CalculatorPart2 {}

impl Calculator for CalculatorPart2 {
    fn calc(params: &GardenParams) -> i32 {
        params.area * params.sides_count
    }
}

// ascii uppercase letters occupy 65-90 code range, we'll use the leftmost
// bit to indicate if this node was visited or not
fn solve<C: Calculator>(field: &mut Field) -> i32 {
    let mut result = 0;
    for row in 0..field.get_row_count() {
        for col in 0..field.get_col_count() {
            if is_node_visited(*field.get(row, col)) {
                continue;
            }
            let params = get_garden_params(
                field,
                &Point {
                    row: row as i32,
                    col: col as i32,
                },
            );
            result += C::calc(&params);
        }
    }

    result
}

#[allow(dead_code)] // used in tests
fn get_test_input() -> Vec<String> {
    vec![
        // cspell: disable
        "RRRRIICCFF".to_string(),
        "RRRRIICCCF".to_string(),
        "VVRRRCCFFF".to_string(),
        "VVRCCCJFFF".to_string(),
        "VVVVCJJCFE".to_string(),
        "VVIVCCJJEE".to_string(),
        "VVIIICJJEE".to_string(),
        "MIIIIIJJEE".to_string(),
        "MIIISIJEEE".to_string(),
        "MMMISSJEEE".to_string(),
        // cspell: enable
    ]
}

#[cfg(test)]
mod tests {
    use crate::day_12::{equal, get_test_input, is_node_visited, visit};

    use super::{get_garden_params, parse_field, Point};

    #[test]
    fn visit_test() {
        let mut node = 'C' as u8;
        assert_eq!(is_node_visited(node), false);
        visit(&mut node);
        assert_eq!(is_node_visited(node), true);

        let another_node = 'C' as u8;

        assert_eq!(equal(node, another_node), true);

        let another_node = 'B' as u8;

        assert_eq!(equal(node, another_node), false);
    }

    #[test]
    fn get_garden_params_test() {
        let mut field = parse_field(get_test_input().iter());
        let params = get_garden_params(&mut field, &Point { row: 0, col: 0 });
        assert_eq!(params.perimeter, 18);
        assert_eq!(params.area, 12);

        let params = get_garden_params(&mut field, &Point { row: 0, col: 4 });
        assert_eq!(params.area, 4);
        assert_eq!(params.perimeter, 8);

        let params = get_garden_params(&mut field, &Point { row: 0, col: 6 });
        assert_eq!(params.area, 14);
        assert_eq!(params.perimeter, 28);

        let params = get_garden_params(&mut field, &Point { row: 4, col: 9 });
        assert_eq!(params.area, 13);
        assert_eq!(params.perimeter, 18);

        let params = get_garden_params(&mut field, &Point { row: 5, col: 2 });
        assert_eq!(params.area, 14);
        assert_eq!(params.perimeter, 22);
    }
}

pub mod part1 {
    use super::{parse_field, solve, CalculatorPart1};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut field = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            solve::<CalculatorPart1>(&mut field).to_string()
        }

        fn day() -> i32 {
            12
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
        use crate::day_12::{get_test_input, parse_field, solve, CalculatorPart1};

        #[test]
        fn solve_test() {
            let mut field = parse_field(get_test_input().iter());
            assert_eq!(solve::<CalculatorPart1>(&mut field), 1930);
        }
    }
}

pub mod part2 {
    use super::{parse_field, solve, CalculatorPart2};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut field = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            solve::<CalculatorPart2>(&mut field).to_string()
        }

        fn day() -> i32 {
            12
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
        use crate::day_12::{get_test_input, parse_field, solve, CalculatorPart2};

        #[test]
        fn solve_test() {
            let mut field = parse_field(get_test_input().iter());
            assert_eq!(solve::<CalculatorPart2>(&mut field), 1206);
        }
    }
}
