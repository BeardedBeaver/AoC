use std::cmp::Ordering;

fn get_max(slice: &str) -> (usize, i64) {
    slice
        .chars()
        .enumerate()
        .max_by(|(_, a), (_, b)| if a > b { Ordering::Greater } else { Ordering::Less })
        .map(|(index, ch)| (index, ch as i64 - 0x30))
        .expect("Max value not found")
}

pub mod part1 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::get_max;

    fn get_joltage(input: &str) -> i64 {
        let (left_idx, left_val) = super::get_max(&input[0..input.len() - 1]);
        let (_, right_val) = get_max(&input[left_idx + 1..]);
        left_val * 10 + right_val
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let file = File::open(file_name).unwrap();
            let reader = BufReader::new(file);

            let mut result = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                result += get_joltage(&line);
            }

            result.to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            3
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day_03::part1::get_joltage;

        #[test]
        fn test_get_joltage() {
            assert_eq!(get_joltage("987654321111111"), 98);
            assert_eq!(get_joltage("811111111111119"), 89);
            assert_eq!(get_joltage("234234234234278"), 78);
            assert_eq!(get_joltage("818181911112111"), 92);
            assert_eq!(get_joltage("111111194311111"), 94);
            assert_eq!(get_joltage("888888888888887"), 88);
        }
    }
}

pub mod part2 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::get_max;

    fn get_joltage(input: &str) -> i64 {
        let mut result = 0;
        let mut start = 0;
        for end in (0..12).rev() {
            let (idx, val) = get_max(&input[start..input.len() - end]);
            result = result * 10 + val;
            start = idx + start + 1;
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let file = File::open(file_name).unwrap();
            let reader = BufReader::new(file);

            let mut result = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                result += get_joltage(&line);
            }

            result.to_string()
        }

        fn year() -> i32 {
            2025
        }

        fn day() -> i32 {
            3
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use super::get_joltage;

        #[test]
        fn test_get_joltage() {
            assert_eq!(get_joltage("987654321111111"), 987654321111);
            assert_eq!(get_joltage("811111111111119"), 811111111119);
            assert_eq!(get_joltage("234234234234278"), 434234234278);
            assert_eq!(get_joltage("818181911112111"), 888911112111);
            assert_eq!(get_joltage("888888888888887"), 888888888888);
        }
    }
}
