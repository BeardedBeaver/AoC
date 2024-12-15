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
    perimeter: u8,
    area: u8,
}

fn get_garden_params(field: &mut Field, origin: &Point) -> GardenParams {
    todo!()
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

// ascii uppercase letters occupy 65-90 code range, we'll use the leftmost
// bit to indicate if this node was visited or not
fn solve(field: &mut Field) -> i32 {
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
            result += params.area as i32 * params.perimeter as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day_12::{equal, is_node_visited, visit};

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
}

pub mod part1 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
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
        #[test]
        fn test() {}
    }
}

pub mod part2 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
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
        #[test]
        fn test() {}
    }
}
