type Point = aoc::Point<i32>;

#[derive(Debug, Default, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse_robots<I, S>(lines: I) -> Vec<Robot>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut result = Vec::new();

    for line in lines {
        if let Some(cap) = re.captures(line.as_ref()) {
            result.push(Robot {
                pos: Point {
                    row: aoc::parse_or_panic(&cap[2]),
                    col: aoc::parse_or_panic(&cap[1]),
                },
                vel: Point {
                    row: aoc::parse_or_panic(&cap[4]),
                    col: aoc::parse_or_panic(&cap[3]),
                },
            });
        }
    }

    result
}

fn move_robot(robot: &mut Robot, field_size: &Point) {
    robot.pos.row += robot.vel.row + field_size.row;
    robot.pos.col += robot.vel.col + field_size.col;

    robot.pos.row %= field_size.row;
    robot.pos.col %= field_size.col;
}

fn count_robots(robots: &Vec<Robot>, field_size: &Point) -> (i32, i32, i32, i32) {
    let mut result = (0, 0, 0, 0);
    for robot in robots.iter() {
        let mut quadrant_row = -1;

        if robot.pos.row < field_size.row / 2 {
            quadrant_row = 0;
        } else if robot.pos.row > field_size.row / 2 {
            quadrant_row = 1;
        };

        let mut quadrant_col = -1;

        if robot.pos.col < field_size.col / 2 {
            quadrant_col = 0;
        } else if robot.pos.col > field_size.col / 2 {
            quadrant_col = 1;
        };

        if quadrant_row < 0 || quadrant_col < 0 {
            continue;
        }

        match (quadrant_row, quadrant_col) {
            (0, 0) => result.0 += 1,
            (0, 1) => result.1 += 1,
            (1, 0) => result.2 += 1,
            (1, 1) => result.3 += 1,
            _ => (),
        }
    }
    result
}

fn get_test_input() -> Vec<String> {
    vec![
        "p=0,4 v=3,-3".to_string(),
        "p=6,3 v=-1,-3".to_string(),
        "p=10,3 v=-1,2".to_string(),
        "p=2,0 v=2,-1".to_string(),
        "p=0,0 v=1,3".to_string(),
        "p=3,0 v=-2,-2".to_string(),
        "p=7,6 v=-1,-3".to_string(),
        "p=3,0 v=-1,-2".to_string(),
        "p=9,3 v=2,3".to_string(),
        "p=7,3 v=-1,2".to_string(),
        "p=2,4 v=2,-3".to_string(),
        "p=9,5 v=-3,-3".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::day_14::{get_test_input, move_robot, Point};

    use super::parse_robots;

    #[test]
    fn parse_robots_test() {
        let lines = get_test_input();

        let robots = parse_robots(lines.iter());
        assert_eq!(robots[0].pos.row, 4);
        assert_eq!(robots[0].pos.col, 0);
        assert_eq!(robots[0].vel.row, -3);
        assert_eq!(robots[0].vel.col, 3);

        assert_eq!(robots[10].pos.row, 4);
        assert_eq!(robots[10].pos.col, 2);
        assert_eq!(robots[10].vel.row, -3);
        assert_eq!(robots[10].vel.col, 2);

        assert_eq!(robots[11].pos.row, 5);
        assert_eq!(robots[11].pos.col, 9);
        assert_eq!(robots[11].vel.row, -3);
        assert_eq!(robots[11].vel.col, -3);
    }

    #[test]
    fn move_robot_test() {
        let lines = vec!["p=2,4 v=2,-3".to_string()];

        let mut robot = parse_robots(lines.iter())[0].clone();
        let field_size = Point { row: 7, col: 11 };
        assert_eq!(robot.pos.row, 4);
        assert_eq!(robot.pos.col, 2);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);

        move_robot(&mut robot, &field_size);

        assert_eq!(robot.pos.row, 1);
        assert_eq!(robot.pos.col, 4);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);

        move_robot(&mut robot, &field_size);

        assert_eq!(robot.pos.row, 5);
        assert_eq!(robot.pos.col, 6);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);

        move_robot(&mut robot, &field_size);

        assert_eq!(robot.pos.row, 2);
        assert_eq!(robot.pos.col, 8);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);

        move_robot(&mut robot, &field_size);

        assert_eq!(robot.pos.row, 6);
        assert_eq!(robot.pos.col, 10);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);

        move_robot(&mut robot, &field_size);

        assert_eq!(robot.pos.row, 3);
        assert_eq!(robot.pos.col, 1);
        assert_eq!(robot.vel.row, -3);
        assert_eq!(robot.vel.col, 2);
    }
}

pub mod part1 {
    use super::{count_robots, move_robot, parse_robots, Point, Robot};

    fn solve(robots: &mut Vec<Robot>, move_count: i32, field_size: &Point) -> (i32, i32, i32, i32) {
        for _ in 0..move_count {
            for i in 0..robots.len() {
                move_robot(&mut robots[i], &field_size);
            }
        }

        count_robots(&robots, &field_size)
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut robots = parse_robots(std::fs::read_to_string(input_file_name).unwrap().lines());
            let field_size = Point { row: 103, col: 101 };
            let count = solve(&mut robots, 100, &field_size);
            (count.0 * count.1 * count.2 * count.3).to_string()
        }

        fn day() -> i32 {
            14
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
        use crate::day_14::{get_test_input, parse_robots, part1::solve, Point};

        #[test]
        fn solve_test() {
            let lines = get_test_input();
            let mut robots = parse_robots(lines.iter());
            let field_size = Point { row: 7, col: 11 };
            let count = solve(&mut robots, 100, &field_size);
            assert_eq!(count, (1, 3, 4, 1));
        }
    }
}

pub mod part2 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
        }

        fn day() -> i32 {
            14
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
