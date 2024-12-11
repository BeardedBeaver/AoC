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

fn get_first_unsafe_index(levels: &[i32]) -> Option<usize> {
    if levels.len() < 2 {
        return None;
    }

    let mut maybe_prev_delta: Option<i32> = None;

    // iterate over adjacent pairs of values
    for (i, chunk) in levels.iter().zip(levels.iter().skip(1)).enumerate() {
        let delta = chunk.0 - chunk.1;
        if delta.abs() < 1 || delta.abs() > 3 {
            return Some(i - 1);
        }
        if let Some(prev_delta) = maybe_prev_delta {
            if delta.signum() != prev_delta.signum() {
                return Some(i - 1);
            }
        }

        maybe_prev_delta = Some(delta);
    }
    None
}

fn is_report_safe(levels: &[i32]) -> bool {
    get_first_unsafe_index(levels).is_none()
}

fn is_report_safe_dampened(levels: &[i32]) -> bool {
    let idx = get_first_unsafe_index(levels);
    if idx.is_none() {
        return true;
    }
    let idx = idx.unwrap();

    let fixed: Vec<i32> = levels[0..idx].iter().chain(levels[idx + 1..].iter()).copied().collect();
    if is_report_safe(&fixed) {
        return true;
    }

    if idx > 0 {
        let fixed: Vec<i32> = levels[0..idx - 1].iter().chain(levels[idx..].iter()).copied().collect();
        if is_report_safe(&fixed) {
            return true;
        }
    }

    let fixed: Vec<i32> = levels[0..idx + 1]
        .iter()
        .chain(levels[idx + 2..].iter())
        .copied()
        .collect();
    if is_report_safe(&fixed) {
        return true;
    }

    false
}

pub mod part1 {
    use super::is_report_safe;
    use super::Report;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let report = Report::from_string(line);
                if is_report_safe(&report.levels) {
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
            let levels = vec![7, 6, 4, 2, 1];
            assert_eq!(is_report_safe(&levels), true);

            let levels = vec![1, 2, 7, 8, 9];

            assert_eq!(is_report_safe(&levels), false);

            let levels = vec![9, 7, 6, 2, 1];
            assert_eq!(is_report_safe(&levels), false);

            let levels = vec![1, 3, 2, 4, 5];
            assert_eq!(is_report_safe(&levels), false);

            let levels = vec![8, 6, 4, 4, 1];
            assert_eq!(is_report_safe(&levels), false);

            let levels = vec![1, 3, 6, 7, 9];
            assert_eq!(is_report_safe(&levels), true);
        }
    }
}

pub mod part2 {
    use super::is_report_safe_dampened;
    use super::Report;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let report = Report::from_string(line);
                if is_report_safe_dampened(&report.levels) {
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

    #[cfg(test)]
    mod tests {
        use crate::day_02::is_report_safe_dampened;

        #[test]
        fn is_report_safe_dampened_test() {
            let levels = vec![7, 6, 4, 2, 1];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![1, 2, 7, 8, 9];
            assert_eq!(is_report_safe_dampened(&levels), false);

            let levels = vec![9, 7, 6, 2, 1];
            assert_eq!(is_report_safe_dampened(&levels), false);

            let levels = vec![1, 3, 2, 4, 5];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![8, 6, 4, 4, 1];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![1, 3, 6, 7, 9];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![20, 8, 6, 4, 1];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![9, 1, 6, 4, 1];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![1, 3, 6, 7, 9, 50];
            assert_eq!(is_report_safe_dampened(&levels), true);

            let levels = vec![57, 55, 56, 58, 59, 62, 65, 68];
            assert_eq!(is_report_safe_dampened(&levels), true);
        }
    }
}
