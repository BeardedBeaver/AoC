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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    #[default]
    Empty,
    Obstacle,
}

#[derive(Default, Clone)]
struct Field<N> {
    nodes: Vec<Vec<N>>,
}

fn parse_field<I, S>(lines: I) -> (Field<Node>, Guard)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut field = Field::<Node>::default();
    let mut guard = Guard::default();
    for (row_idx, line) in lines.enumerate() {
        let mut row = Vec::with_capacity(line.as_ref().len());
        for (col_idx, c) in line.as_ref().chars().enumerate() {
            match c {
                '.' => row.push(Node::Empty),
                '#' => row.push(Node::Obstacle),
                _ => {
                    row.push(Node::Empty);
                    guard.pos = Point {
                        row: row_idx as i32,
                        col: col_idx as i32,
                    };
                    guard.direction = match c {
                        '^' => Direction::North,
                        '>' => Direction::East,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        _ => unreachable!(),
                    }
                }
            };
        }
        field.nodes.push(row);
    }
    (field, guard)
}

#[derive(Debug, PartialEq, Eq)]
enum TraverseResult {
    Exited,
    Stuck,
}

fn traverse(field: &Field<Node>, guard: &Guard) -> (Field<i32>, TraverseResult) {
    let mut guard = *guard;
    let mut path = Field::<i32>::default();

    // initialize path matrix
    for _ in 0..field.nodes.len() {
        path.nodes.push(vec![0; field.nodes[0].len()]);
    }

    loop {
        path.nodes[guard.pos.row as usize][guard.pos.col as usize] |= guard.direction as i32;

        let mut new_guard_pos = guard.pos;
        match guard.direction {
            Direction::North => new_guard_pos.row = guard.pos.row - 1,
            Direction::West => new_guard_pos.col = guard.pos.col - 1,
            Direction::South => new_guard_pos.row = guard.pos.row + 1,
            Direction::East => new_guard_pos.col = guard.pos.col + 1,
            _ => unreachable!(),
        }

        if new_guard_pos.row < 0
            || new_guard_pos.row >= field.nodes.len() as i32
            || new_guard_pos.col < 0
            || new_guard_pos.col >= field.nodes[0].len() as i32
        {
            return (path, TraverseResult::Exited);
        }

        if field.nodes[new_guard_pos.row as usize][new_guard_pos.col as usize] == Node::Obstacle {
            match guard.direction {
                Direction::North => guard.direction = Direction::East,
                Direction::East => guard.direction = Direction::South,
                Direction::South => guard.direction = Direction::West,
                Direction::West => guard.direction = Direction::North,
                _ => unreachable!(),
            }
        } else {
            guard.pos = new_guard_pos;
        }

        let path_node = &path.nodes[guard.pos.row as usize][guard.pos.col as usize];
        if *path_node & (guard.direction as i32) != 0 {
            return (path, TraverseResult::Stuck);
        }
    }
}

fn count_visited_nodes(path: &Field<i32>) -> i32 {
    let mut result = 0;
    for row in path.nodes.iter() {
        for node in row.iter() {
            if *node != 0 {
                result += 1;
            }
        }
    }
    result
}

#[allow(dead_code)] // used in tests
fn get_test_filed() -> Vec<String> {
    vec![
        "....#.....".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        "..#.......".to_string(),
        ".......#..".to_string(),
        "..........".to_string(),
        ".#..^.....".to_string(),
        "........#.".to_string(),
        "#.........".to_string(),
        "......#...".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::{count_visited_nodes, get_test_filed, parse_field, traverse, Direction, Node, Point, TraverseResult};

    #[test]
    fn test_parse_field() {
        let (field, guard) = parse_field(get_test_filed().iter());

        assert_eq!(field.nodes.len(), 10);
        for row in field.nodes.iter() {
            assert_eq!(row.len(), 10);
        }

        assert_eq!(field.nodes[0][0], Node::Empty);
        assert_eq!(field.nodes[0][4], Node::Obstacle);
        assert_eq!(field.nodes[9][6], Node::Obstacle);

        assert_eq!(guard.pos, Point { row: 6, col: 4 });
        assert_eq!(guard.direction, Direction::North);
    }

    #[test]
    fn test_traverse() {
        let (field, guard) = parse_field(get_test_filed().iter());
        let (path, result) = traverse(&field, &guard);

        assert_eq!(result, TraverseResult::Exited);
        assert_eq!(count_visited_nodes(&path), 41);
    }

    #[test]
    fn test_traverse_edge_case() {
        {
            let lines = vec!["###", "#^#", "###"];
            let (field, guard) = parse_field(lines.iter());
            let (_, result) = traverse(&field, &guard);

            assert_eq!(result, TraverseResult::Stuck);
        }
        {
            let lines = vec!["###", "#^#", "#.#"];
            let (field, guard) = parse_field(lines.iter());
            let (_, result) = traverse(&field, &guard);

            assert_eq!(result, TraverseResult::Exited);
        }
    }
}

pub mod part1 {
    use crate::day_06::TraverseResult;

    use super::{count_visited_nodes, parse_field, traverse};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (field, guard) = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            let (path, traverse_result) = traverse(&field, &guard);
            assert_eq!(traverse_result, TraverseResult::Exited);
            count_visited_nodes(&path).to_string()
        }

        fn day() -> i32 {
            6
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
    use crate::day_06::{parse_field, traverse};

    use super::{Field, Guard, Node, TraverseResult};

    fn count_obstacles(field: &Field<Node>, guard: &Guard) -> i32 {
        let (path, _) = traverse(&field, &guard);
        let mut result = 0;
        for (row_idx, row) in path.nodes.iter().enumerate() {
            for (col_idx, node) in row.iter().enumerate() {
                if *node == 0 || (guard.pos.row == row_idx as i32 && guard.pos.col == col_idx as i32) {
                    continue;
                }
                let mut altered_field = field.clone();
                altered_field.nodes[row_idx][col_idx] = Node::Obstacle;
                let (_, traverse_result) = traverse(&altered_field, &guard);
                if traverse_result == TraverseResult::Stuck {
                    result += 1;
                }
            }
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (field, guard) = parse_field(std::fs::read_to_string(input_file_name).unwrap().lines());
            count_obstacles(&field, &guard).to_string()
        }

        fn day() -> i32 {
            6
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
        use crate::day_06::{get_test_filed, parse_field, part2::count_obstacles};

        #[test]
        fn test_solve() {
            let (field, guard) = parse_field(get_test_filed().iter());
            assert_eq!(count_obstacles(&field, &guard), 6);
        }
    }
}
