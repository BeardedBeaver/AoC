#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RotationResult {
    pos: i32,
    zero_count: u32,
}

fn parse_line(line: &str) -> Rotation {
    let dir = match line.chars().nth(0) {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        _ => unreachable!(),
    };
    let dist = aoc::parse_or_panic::<i32>(&line[1..]);
    Rotation {
        direction: dir,
        distance: dist,
    }
}

fn rotate(pos: i32, rot: Rotation, max: i32) -> RotationResult {
    let mut pos = pos;
    let mut zero_count = 0;
    let delta = match rot.direction {
        Direction::Left => -1,
        Direction::Right => 1,
    };
    for _ in 0..rot.distance {
        pos += delta;
        if pos > 99 {
            pos -= 100;
        }
        if pos < 0 {
            pos += 100;
        }
        if pos == 0 {
            zero_count += 1;
        }
    }

    RotationResult { pos, zero_count }
}

#[cfg(test)]
mod tests {
    use crate::day_01::{rotate, Direction, Rotation, RotationResult};

    #[test]
    fn rotate_left_test() {
        let pos = 5;
        let max = 99;

        let rot = Rotation {
            direction: Direction::Left,
            distance: 2,
        };
        assert_eq!(rotate(pos, rot, max), RotationResult { pos: 3, zero_count: 0 });

        let rot = Rotation {
            direction: Direction::Left,
            distance: 5,
        };
        assert_eq!(rotate(pos, rot, max), RotationResult { pos: 0, zero_count: 1 });

        let rot = Rotation {
            direction: Direction::Left,
            distance: 10,
        };
        assert_eq!(rotate(pos, rot, max), RotationResult { pos: 95, zero_count: 1 });
    }

    #[test]
    fn rotate_right_test() {
        let pos = 5;
        let max = 99;

        let rot = Rotation {
            direction: Direction::Right,
            distance: 20,
        };
        assert_eq!(rotate(pos, rot, max), RotationResult { pos: 25, zero_count: 0 });

        let pos = 90;
        let rot = Rotation {
            direction: Direction::Right,
            distance: 25,
        };
        assert_eq!(rotate(pos, rot, max), RotationResult { pos: 15, zero_count: 1 });
    }
}

pub mod part1 {
    use crate::day_01::{parse_line, rotate, Rotation};

    fn solve(input: &[Rotation]) -> u32 {
        let mut count = 0;
        let mut pos = 50;
        let max = 99;
        for rotation in input.iter() {
            let result = rotate(pos, rotation.clone(), 99);
            pos = result.pos;
            if pos == 0 {
                count += 1;
            }
        }
        count
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut input = Vec::new();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                input.push(parse_line(&line));
            }

            let result = solve(&input);
            result.to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            1
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_01::{parse_line, part1::solve, rotate, Direction, Rotation};

        #[test]
        fn test_solve() {
            let input = vec![
                parse_line("L68"),
                parse_line("L30"),
                parse_line("R48"),
                parse_line("L5"),
                parse_line("R60"),
                parse_line("L55"),
                parse_line("L1"),
                parse_line("L99"),
                parse_line("R14"),
                parse_line("L82"),
            ];
            assert_eq!(solve(&input), 3);
        }

        #[test]
        fn test_multiple_rotations() {
            let rot = Rotation {
                direction: Direction::Right,
                distance: 1000,
            };
            let result = rotate(50, rot, 99);
            assert_eq!(result.pos, 50);
            assert_eq!(result.zero_count, 10);
        }
    }
}
pub mod part2 {
    use crate::day_01::{parse_line, rotate, Rotation};

    fn solve(input: &[Rotation]) -> u32 {
        let mut count = 0;
        let mut pos = 50;
        let max = 99;
        for rotation in input.iter() {
            let result = rotate(pos, rotation.clone(), 99);
            pos = result.pos;
            count += result.zero_count;
        }
        count
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut input = Vec::new();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                input.push(parse_line(&line));
            }

            let result = solve(&input);
            result.to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            1
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_01::{parse_line, part2::solve, Rotation};

        #[test]
        fn test_solve() {
            let input = vec![
                parse_line("L68"),
                parse_line("L30"),
                parse_line("R48"),
                parse_line("L5"),
                parse_line("R60"),
                parse_line("L55"),
                parse_line("L1"),
                parse_line("L99"),
                parse_line("R14"),
                parse_line("L82"),
            ];
            assert_eq!(solve(&input), 6);
        }
    }
}
