struct Machine {
    // button rules
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,

    // target
    tx: i32,
    ty: i32,
}

fn solve(machine: &Machine) -> Option<(i32, i32)> {
    let mut a = 0;
    let mut b = 0;

    let mut tx = machine.tx;
    let mut ty = machine.ty;

    loop {
        b += 1;
        if b >= 100 {
            return None;
        }

        tx -= machine.bx;
        ty -= machine.by;

        if tx % machine.ax == 0 && ty % machine.ay == 0 {
            let x = tx / machine.ax;
            let y = ty / machine.ay;
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
}

pub mod part1 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
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
