#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Machine {
    // button rules
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,

    // target
    tx: i64,
    ty: i64,
}

use regex::Regex;

fn parse_input<I, S>(lines: I) -> Vec<Machine>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = Vec::new();
    let mut m = Machine::default();

    let button_a_re = Regex::new(r"^Button A: X[+=](\d+), Y[+=](\d+)$").unwrap();
    let button_b_re = Regex::new(r"^Button B: X[+=](\d+), Y[+=](\d+)$").unwrap();
    let prize_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    for line in lines {
        let line = line.as_ref().trim();
        if let Some(captures) = button_a_re.captures(line) {
            m.ax = aoc::parse_or_panic(&captures[1]);
            m.ay = aoc::parse_or_panic(&captures[2]);
        } else if let Some(captures) = button_b_re.captures(line) {
            m.bx = aoc::parse_or_panic(&captures[1]);
            m.by = aoc::parse_or_panic(&captures[2]);
        } else if let Some(captures) = prize_re.captures(line) {
            m.tx = aoc::parse_or_panic(&captures[1]);
            m.ty = aoc::parse_or_panic(&captures[2]);
        } else if line.is_empty() {
            result.push(m);
        }
    }
    result.push(m);

    result
}

fn solve(machine: &Machine) -> Option<(i64, i64)> {
    let mut a: i64 = 0;
    let mut b: i64 = 0;

    let mut tx = machine.tx;
    let mut ty = machine.ty;

    loop {
        b += 1;
        if machine.tx < 100000 && b >= 100 {
            return None;
        }

        tx -= machine.bx as i64;
        ty -= machine.by as i64;

        if tx < 0 || ty < 0 {
            return None;
        }

        if tx % machine.ax as i64 == 0 && ty % machine.ay as i64 == 0 {
            let x = tx / machine.ax as i64;
            let y = ty / machine.ay as i64;
            if x == y {
                a = y;
                break;
            }
        }
    }

    Some((a, b))
}

#[cfg(test)]
mod tests {
    use crate::day_13::{solve, Machine};

    use super::parse_input;

    #[test]
    fn solve_test() {
        let machine = Machine {
            ax: 94,
            ay: 34,
            bx: 22,
            by: 67,
            tx: 8400,
            ty: 5400,
        };
        assert_eq!(solve(&machine), Some((80, 40)));

        let machine = Machine {
            ax: 26,
            ay: 66,
            bx: 67,
            by: 21,
            tx: 12748,
            ty: 12176,
        };
        assert_eq!(solve(&machine), None);

        let machine = Machine {
            ax: 17,
            ay: 86,
            bx: 84,
            by: 37,
            tx: 7870,
            ty: 6450,
        };
        assert_eq!(solve(&machine), Some((38, 86)));

        let machine = Machine {
            ax: 69,
            ay: 23,
            bx: 27,
            by: 71,
            tx: 18641,
            ty: 10279,
        };
        assert_eq!(solve(&machine), None);
    }

    #[test]
    fn solve_test_pt2() {
        let machine = Machine {
            ax: 26,
            ay: 66,
            bx: 67,
            by: 21,
            tx: 10000000012748,
            ty: 10000000012176,
        };
        let huh = solve(&machine);
        println!("{:?}", huh);
        assert!(false);
    }

    #[test]
    fn parse_input_test() {
        let lines = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
            "",
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
            "",
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
            "",
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ];
        let machines = parse_input(lines.iter());

        let expected = vec![
            Machine {
                ax: 94,
                ay: 34,
                bx: 22,
                by: 67,
                tx: 8400,
                ty: 5400,
            },
            Machine {
                ax: 26,
                ay: 66,
                bx: 67,
                by: 21,
                tx: 12748,
                ty: 12176,
            },
            Machine {
                ax: 17,
                ay: 86,
                bx: 84,
                by: 37,
                tx: 7870,
                ty: 6450,
            },
            Machine {
                ax: 69,
                ay: 23,
                bx: 27,
                by: 71,
                tx: 18641,
                ty: 10279,
            },
        ];

        for (actual, expected) in machines.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }
    }
}

pub mod part1 {
    use super::{parse_input, solve};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let machines = parse_input(std::fs::read_to_string(input_file_name).unwrap().lines());
            let mut result = 0;
            for machine in machines.iter() {
                if let Some(pushes) = solve(&machine) {
                    result += 3 * pushes.0 + pushes.1;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            13
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
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
        }

        fn day() -> i32 {
            13
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }
}
