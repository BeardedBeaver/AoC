#[derive(Debug, Default)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from_string(line: &str) -> Report {
        let mut result = Report::default();
        result.levels = line.split(' ').map(|word| aoc::parse_or_panic(word)).collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_from_string_test() {
        let line = "7 6 4 2 1";
        let report = Report::from_string(line);
        assert_eq!(report.levels, vec![7, 6, 4, 2, 1]);
    }

    #[test]
    fn is_report_safe_test() {
        let report = Report {
            levels: vec![7, 6, 4, 2, 1],
        };
        assert_eq!(is_report_safe(&report), true);

        let report = Report {
            levels: vec![1, 2, 7, 8, 9],
        };
        assert_eq!(is_report_safe(&report), false);

        let report = Report {
            levels: vec![9, 7, 6, 2, 1],
        };
        assert_eq!(is_report_safe(&report), false);

        let report = Report {
            levels: vec![1, 3, 2, 4, 5],
        };
        assert_eq!(is_report_safe(&report), false);

        let report = Report {
            levels: vec![8, 6, 4, 4, 1],
        };
        assert_eq!(is_report_safe(&report), false);

        let report = Report {
            levels: vec![1, 3, 6, 7, 9],
        };
        assert_eq!(is_report_safe(&report), true);
    }
}

fn is_report_safe(report: &Report) -> bool {
    if report.levels.len() < 2 {
        return true;
    }

    let mut maybe_prev_delta: Option<i32> = None;

    for i in 1..report.levels.len() {
        let delta = report.levels[i - 1] - report.levels[i];
        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
        if let Some(prev_delta) = maybe_prev_delta {
            if delta.signum() != prev_delta.signum() {
                return false;
            }
        }

        maybe_prev_delta = Some(delta);
    }
    true
}

pub mod part1 {
    use super::Report;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let report = Report::from_string(line);
                if super::is_report_safe(&report) {
                    result += 1;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            2
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
    use super::Report;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let report = Report::from_string(line);
                if super::is_report_safe(&report) {
                    result += 1;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            2
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }
}
