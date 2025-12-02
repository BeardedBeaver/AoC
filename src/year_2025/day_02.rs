#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

fn parse_range(s: &str) -> Option<Range> {
    let parts = s.split("-").collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    Some(Range {
        start: aoc::parse_or_panic::<i64>(parts[0]),
        end: aoc::parse_or_panic::<i64>(parts[1]),
    })
}

fn parse_input_line(s: &str) -> Vec<Range> {
    s.split(",")
        .map(|slice| {
            let slice = slice.trim();
            parse_range(slice).expect("couldn't parse range")
        })
        .collect::<Vec<Range>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_range, Range};

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("356-567"), Some(Range { start: 356, end: 567 }));
        assert_eq!(parse_range("234"), None);
    }
}

pub mod part1 {
    use super::{parse_input_line, Range};

    fn get_parts_from_number(input: i64) -> Option<(i64, i64)> {
        let power = input.checked_ilog10()? + 1;
        if power % 2 == 1 {
            return None;
        }
        let half = 10i64.pow(power / 2);
        Some((input / half, input % half))
    }

    fn solve(input: &[Range]) -> i64 {
        let mut result = 0;

        for range in input.iter() {
            for i in range.start..=range.end {
                if let Some((first, second)) = get_parts_from_number(i) {
                    if first == second {
                        result += i;
                    }
                }
            }
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut input = parse_input_line(&std::fs::read_to_string(file_name).unwrap());
            solve(&input).to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            2
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::get_parts_from_number;
        use crate::day_02::{parse_input_line, parse_range, part1::solve, Range};

        #[test]
        fn test_get_parts_from_number() {
            assert_eq!(get_parts_from_number(2525), Some((25, 25)));
            assert_eq!(get_parts_from_number(69), Some((6, 9)));
            assert_eq!(get_parts_from_number(125), None);
            assert_eq!(get_parts_from_number(12346789), Some((1234, 6789)));
            assert_eq!(get_parts_from_number(1000), Some((10, 0)));
        }

        #[test]
        fn test_solve() {
            let input = concat!(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
                "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
                "824824821-824824827,2121212118-2121212124",
            );
            let ranges = parse_input_line(input);
            assert_eq!(solve(&ranges), 1227775554);
        }
    }
}

pub mod part2 {
    use crate::day_02::{parse_input_line, Range};

    fn folds(id: i64, fold_factor: u32) -> bool {
        let mut id = id;
        let mut part: Option<i64> = None;
        let pow = 10i64.pow(fold_factor);
        while id > 0 {
            let current_part = id % pow;
            match part {
                Some(p) => {
                    if p != current_part {
                        return false;
                    }
                }
                None => part = Some(current_part),
            }
            id /= pow;
        }

        true
    }

    fn is_id_invalid(id: i64) -> bool {
        let power = id.checked_ilog10().unwrap() + 1;
        let to = power / 2;
        let target = id.to_string();
        for i in 1..=to {
            if power % i != 0 {
                continue;
            }
            if folds(id, i) {
                return true;
            }
        }
        false
    }

    fn solve(input: &[Range]) -> i64 {
        let mut result = 0;

        for range in input.iter() {
            for i in range.start..=range.end {
                if is_id_invalid(i) {
                    result += i;
                }
            }
        }

        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut input = parse_input_line(&std::fs::read_to_string(file_name).unwrap());
            solve(&input).to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            2
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_02::{
            parse_input_line, parse_range,
            part2::{is_id_invalid, solve},
            Range,
        };

        #[test]
        fn test_is_id_invalid() {
            assert_eq!(is_id_invalid(11), true);
            assert_eq!(is_id_invalid(111), true);
            assert_eq!(is_id_invalid(121), false);
            assert_eq!(is_id_invalid(100), false);
            assert_eq!(is_id_invalid(121212), true);
            assert_eq!(is_id_invalid(125212), false);
            assert_eq!(is_id_invalid(10001000), true);
            assert_eq!(is_id_invalid(10101010), true);
            assert_eq!(is_id_invalid(50505), false);
        }

        #[test]
        fn test_solve() {
            let input = concat!(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
                "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
                "824824821-824824827,2121212118-2121212124",
            );
            let ranges = parse_input_line(input);
            assert_eq!(solve(&ranges), 4174379265);
        }
    }
}
