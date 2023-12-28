#[derive(Debug, Default)]
struct Node {
    kind: char,
    energised: u8,
}

#[derive(Debug, Default)]
struct Field {
    nodes: Vec<Vec<Node>>,
    column_count: usize,
}

enum Direction {
    North,
    West,
    South,
    East,
}

struct Beam {
    row: i32,
    col: i32,
    direction: Direction,
}

impl Field {
    fn from_file(file_name: &str) -> Field {
        let mut result = Field::default();

        for s in std::fs::read_to_string(file_name).unwrap().lines() {
            let mut line = Vec::with_capacity(s.len());
            result.column_count = s.len();
            for c in s.as_bytes() {
                line.push(Node {
                    kind: *c as char,
                    energised: 0,
                })
            }
            result.nodes.push(line);
        }

        result
    }

    fn traverse(self: &mut Field, start_row: i32, start_col: i32) {
        assert!(!self.nodes.is_empty());
        let mut beams = vec![Beam {
            row: start_row,
            col: start_col,
            direction: Direction::East,
        }];

        while !beams.is_empty() {
            // remove all beams that went out of bounds
            beams.retain(|beam| {
                beam.row >= 0
                    && beam.col >= 0
                    && beam.row < self.nodes.len() as i32
                    && beam.col < self.column_count as i32
            });

            // reflect all beams
            let mut reflected = Vec::default();
            for beam in beams.iter() {
                match self.nodes[beam.row as usize][beam.col as usize] {
                    &['\\', '/', '|', '-'] => reflected.push(beam.clone()),
                    _ => ,
                }
            }

            // remove all beams that were reflected

            // remove all beams that are going over the same path

            // advance all beams
            for beam in beams.iter_mut() {
                match &beam.direction {
                    Direction::North => beam.row -= 1,
                    Direction::West => beam.row -= 1,
                    Direction::South => beam.col += 1,
                    Direction::East => beam.col += 1,
                }
            }

            // mark fields under beams
        }
    }

    fn energized(self: &Field) -> u64 {
        let mut result = 0;
        for line in self.nodes.iter() {
            for node in line.iter() {
                if node.energised != 0 {
                    result += 1;
                }
            }
        }
        result
    }
}

pub mod part1 {
    use super::*;

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let field = Field::from_file(file_name);
            let mut result = 0;

            result.to_string()
        }

        fn day() -> i32 {
            16
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
