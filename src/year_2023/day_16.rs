use aoc::Direction;

#[derive(Debug, Default, Clone)]
struct Node {
    kind: char,
    energized: u32,
}

#[derive(Debug, Default, Clone)]
struct Field {
    nodes: Vec<Vec<Node>>,
    column_count: usize,
}

#[derive(Clone, Copy)]
struct Beam {
    row: i32,
    col: i32,
    direction: Direction,
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
            result.column_count = s.len();
            for c in s.as_bytes() {
                line.push(Node {
                    kind: *c as char,
                    energized: 0,
                })
            }
            result.nodes.push(line);
        }

        result
    }

    fn traverse(self: &mut Field, mut beams: Vec<Beam>) {
        assert!(!self.nodes.is_empty());

        while !beams.is_empty() {
            // energize fields under beams
            for beam in beams.iter() {
                self.nodes[beam.row as usize][beam.col as usize].energized =
                    self.nodes[beam.row as usize][beam.col as usize].energized | beam.direction as u32;
            }

            let mut next_beams: Vec<Beam> = Vec::with_capacity(beams.capacity());

            // advance all beams
            for beam in beams.iter() {
                match self.nodes[beam.row as usize][beam.col as usize].kind {
                    // just pass forward
                    '.' => match &beam.direction {
                        Direction::North => next_beams.push(Beam {
                            row: beam.row - 1,
                            col: beam.col,
                            direction: beam.direction,
                        }),
                        Direction::West => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col - 1,
                            direction: beam.direction,
                        }),
                        Direction::South => next_beams.push(Beam {
                            row: beam.row + 1,
                            col: beam.col,
                            direction: beam.direction,
                        }),
                        Direction::East => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col + 1,
                            direction: beam.direction,
                        }),
                        _ => unreachable!(),
                    },
                    // vertical splitter (reflect north and south)
                    '|' => match &beam.direction {
                        Direction::West | Direction::East => {
                            next_beams.push(Beam {
                                row: beam.row - 1,
                                col: beam.col,
                                direction: Direction::North,
                            });
                            next_beams.push(Beam {
                                row: beam.row + 1,
                                col: beam.col,
                                direction: Direction::South,
                            })
                        }
                        Direction::North => next_beams.push(Beam {
                            row: beam.row - 1,
                            col: beam.col,
                            direction: beam.direction,
                        }),
                        Direction::South => next_beams.push(Beam {
                            row: beam.row + 1,
                            col: beam.col,
                            direction: beam.direction,
                        }),
                        _ => unreachable!(),
                    },
                    // horizontal splitter (reflect west and east)
                    '-' => match &beam.direction {
                        Direction::North | Direction::South => {
                            next_beams.push(Beam {
                                row: beam.row,
                                col: beam.col - 1,
                                direction: Direction::West,
                            });
                            next_beams.push(Beam {
                                row: beam.row,
                                col: beam.col + 1,
                                direction: Direction::East,
                            })
                        }
                        Direction::West => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col - 1,
                            direction: beam.direction,
                        }),
                        Direction::East => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col + 1,
                            direction: beam.direction,
                        }),
                        _ => unreachable!(),
                    },
                    // diagonal mirror
                    '/' => match &beam.direction {
                        Direction::North => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col + 1,
                            direction: Direction::East,
                        }),
                        Direction::South => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col - 1,
                            direction: Direction::West,
                        }),
                        Direction::East => next_beams.push(Beam {
                            row: beam.row - 1,
                            col: beam.col,
                            direction: Direction::North,
                        }),
                        Direction::West => next_beams.push(Beam {
                            row: beam.row + 1,
                            col: beam.col,
                            direction: Direction::South,
                        }),
                        _ => unreachable!(),
                    },

                    // another diagonal mirror
                    '\\' => match &beam.direction {
                        Direction::North => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col - 1,
                            direction: Direction::West,
                        }),
                        Direction::South => next_beams.push(Beam {
                            row: beam.row,
                            col: beam.col + 1,
                            direction: Direction::East,
                        }),
                        Direction::East => next_beams.push(Beam {
                            row: beam.row + 1,
                            col: beam.col,
                            direction: Direction::South,
                        }),
                        Direction::West => next_beams.push(Beam {
                            row: beam.row - 1,
                            col: beam.col,
                            direction: Direction::North,
                        }),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }

            // remove all beams that went out of bounds
            next_beams.retain(|beam| {
                beam.row >= 0
                    && beam.col >= 0
                    && beam.row < self.nodes.len() as i32
                    && beam.col < self.column_count as i32
            });

            // remove all beams that are going over the same path
            next_beams
                .retain(|beam| self.nodes[beam.row as usize][beam.col as usize].energized & beam.direction as u32 == 0);

            // reassign beams
            beams = next_beams;
        }
    }

    fn score(self: &Field) -> u64 {
        let mut result = 0;
        for line in self.nodes.iter() {
            for node in line.iter() {
                if node.energized != 0 {
                    result += 1;
                }
            }
        }
        result
    }
}

pub mod part1 {
    use super::*;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut field = Field::from_file(file_name);

            let beams = vec![Beam {
                row: 0,
                col: 0,
                direction: Direction::East,
            }];

            field.traverse(beams);
            field.score().to_string()
        }

        fn day() -> i32 {
            16
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
        use super::*;

        #[test]
        fn from_lines_test() {
            let lines = vec!["\\/|-", "..-.", "....", "|-|-"];
            let field = Field::from_lines(&lines);

            assert_eq!(field.nodes[0][0].kind, '\\');
            assert_eq!(field.nodes[0][1].kind, '/');
            assert_eq!(field.nodes[0][2].kind, '|');
            assert_eq!(field.nodes[0][3].kind, '-');

            assert_eq!(field.nodes[1][0].kind, '.');
            assert_eq!(field.nodes[1][1].kind, '.');
            assert_eq!(field.nodes[1][2].kind, '-');
            assert_eq!(field.nodes[1][3].kind, '.');

            assert_eq!(field.nodes[2][0].kind, '.');
            assert_eq!(field.nodes[2][1].kind, '.');
            assert_eq!(field.nodes[2][2].kind, '.');
            assert_eq!(field.nodes[2][3].kind, '.');

            assert_eq!(field.nodes[3][0].kind, '|');
            assert_eq!(field.nodes[3][1].kind, '-');
            assert_eq!(field.nodes[3][2].kind, '|');
            assert_eq!(field.nodes[3][3].kind, '-');
        }

        #[test]
        fn traverse_east_simple_test() {
            let lines = vec!["....", "....", "....", "...."];
            let mut field = Field::from_lines(&lines);
            let beams = vec![Beam {
                row: 0,
                col: 0,
                direction: Direction::East,
            }];

            field.traverse(beams);

            assert_eq!(field.nodes[0][0].energized, 0x8);
            assert_eq!(field.nodes[0][1].energized, 0x8);
            assert_eq!(field.nodes[0][2].energized, 0x8);
            assert_eq!(field.nodes[0][3].energized, 0x8);

            for i in 1..4 as usize {
                assert_eq!(field.nodes[i][0].energized, 0);
                assert_eq!(field.nodes[i][1].energized, 0);
                assert_eq!(field.nodes[i][2].energized, 0);
                assert_eq!(field.nodes[i][3].energized, 0);
            }
        }

        #[test]
        fn traverse_south_simple_test() {
            let lines = vec!["....", "....", "....", "...."];
            let mut field = Field::from_lines(&lines);
            let beams = vec![Beam {
                row: 0,
                col: 1, // Starting from the second column
                direction: Direction::South,
            }];

            field.traverse(beams);

            assert_eq!(field.nodes[0][1].energized, 0x4);
            assert_eq!(field.nodes[1][1].energized, 0x4);
            assert_eq!(field.nodes[2][1].energized, 0x4);
            assert_eq!(field.nodes[3][1].energized, 0x4);

            for i in 0..4 as usize {
                assert_eq!(field.nodes[i][0].energized, 0);
                assert_eq!(field.nodes[i][2].energized, 0);
                assert_eq!(field.nodes[i][3].energized, 0);
            }
        }

        #[test]
        fn traverse_west_simple_test() {
            let lines = vec!["....", "....", "....", "...."];
            let mut field = Field::from_lines(&lines);
            let beams = vec![Beam {
                row: 2, // Starting from the third row
                col: 3, // Starting from the fourth column
                direction: Direction::West,
            }];

            field.traverse(beams);

            assert_eq!(field.nodes[2][3].energized, 0x2); // Energized by westward beam
            assert_eq!(field.nodes[2][2].energized, 0x2);
            assert_eq!(field.nodes[2][1].energized, 0x2);
            assert_eq!(field.nodes[2][0].energized, 0x2);

            // Ensure other nodes in the grid are not energized
            for i in 0..4 as usize {
                if i != 2 {
                    assert_eq!(field.nodes[i][0].energized, 0);
                    assert_eq!(field.nodes[i][1].energized, 0);
                    assert_eq!(field.nodes[i][2].energized, 0);
                    assert_eq!(field.nodes[i][3].energized, 0);
                }
            }
        }

        #[test]
        fn traverse_north_simple_test() {
            let lines = vec!["....", "....", "....", "...."];
            let mut field = Field::from_lines(&lines);
            let beams = vec![Beam {
                row: 3, // Starting from the bottom row (fourth row)
                col: 1, // Starting from the second column
                direction: Direction::North,
            }];

            field.traverse(beams);

            assert_eq!(field.nodes[3][1].energized, 0x1); // Energized by northward beam
            assert_eq!(field.nodes[2][1].energized, 0x1);
            assert_eq!(field.nodes[1][1].energized, 0x1);
            assert_eq!(field.nodes[0][1].energized, 0x1);

            // Ensure other nodes in the grid are not energized
            for i in 0..4 as usize {
                assert_eq!(field.nodes[i][0].energized, 0);
                assert_eq!(field.nodes[i][2].energized, 0);
                assert_eq!(field.nodes[i][3].energized, 0);
            }
        }

        #[test]
        fn traverse_example_test() {
            let lines = vec![
                ".|...\\....",
                "|.-.\\.....",
                ".....|-...",
                "........|.",
                "..........",
                ".........\\",
                "..../.\\\\..",
                ".-.-/..|..",
                ".|....-|.\\",
                "..//.|....",
            ];
            let mut field = Field::from_lines(&lines);
            let beams = vec![Beam {
                row: 0,
                col: 0,
                direction: Direction::East,
            }];

            field.traverse(beams);

            assert_eq!(field.score(), 46);
        }
    }
}

pub mod part2 {
    use super::*;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let field = Field::from_file(file_name);
            let mut result: u64 = 0;

            let col_count = field.column_count;
            let row_count = field.nodes.len();

            // from north to south
            for i in 0..col_count {
                let mut current_field = field.clone();
                let beams = vec![Beam {
                    row: 0,
                    col: i as i32,
                    direction: Direction::South,
                }];

                current_field.traverse(beams);
                result = std::cmp::max(result, current_field.score());
            }

            // from south to north
            for i in 0..col_count {
                let mut current_field = field.clone();
                let beams = vec![Beam {
                    row: (row_count - 1) as i32,
                    col: i as i32,
                    direction: Direction::North,
                }];

                current_field.traverse(beams);
                result = std::cmp::max(result, current_field.score());
            }

            // from west to east
            for i in 0..row_count {
                let mut current_field = field.clone();
                let beams = vec![Beam {
                    row: i as i32,
                    col: 0,
                    direction: Direction::East,
                }];

                current_field.traverse(beams);
                result = std::cmp::max(result, current_field.score());
            }

            // from west to east
            for i in 0..row_count {
                let mut current_field = field.clone();
                let beams = vec![Beam {
                    row: i as i32,
                    col: (col_count - 1) as i32,
                    direction: Direction::East,
                }];

                current_field.traverse(beams);
                result = std::cmp::max(result, current_field.score());
            }

            result.to_string()
        }

        fn day() -> i32 {
            16
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }
}
