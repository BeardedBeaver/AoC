fn parse_input_line(line: &str) -> Vec<Option<i32>> {
    let mut result = Vec::with_capacity(line.len());
    for (idx, c) in line.chars().enumerate() {
        let value_to_add = if idx % 2 == 0 { Some((idx / 2) as i32) } else { None };
        for _ in 0..c.to_digit(10).unwrap() {
            result.push(value_to_add);
        }
    }
    result
}

fn compact(disk_line: &mut Vec<Option<i32>>) {
    let mut lp = 0;
    let mut rp = disk_line.len() - 1;

    while lp < rp {
        if disk_line[lp].is_some() {
            lp += 1;
        } else if disk_line[rp].is_none() {
            rp -= 1;
        } else {
            disk_line.swap(lp, rp);
            lp += 1;
            rp -= 1;
        }
    }
}

fn get_checksum(disk_line: &Vec<Option<i32>>) -> u64 {
    let mut result = 0;

    for (idx, maybe_value) in disk_line.iter().enumerate() {
        if maybe_value.is_none() {
            break;
        }
        result += idx as u64 * maybe_value.unwrap() as u64;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day_09::get_checksum;

    use super::{compact, parse_input_line};

    #[test]
    fn parse_input_line_test() {
        let input = "12345";
        let result = parse_input_line(input);
        assert_eq!(
            result,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );

        let input = "2333133121414131402";
        let result = parse_input_line(input);
        assert_eq!(
            result,
            vec![
                Some(0),
                Some(0),
                None,
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                Some(4),
                Some(4),
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                Some(9),
                Some(9),
            ]
        );
    }

    #[test]
    fn compact_test() {
        let line = "12345";
        let mut disk_line = parse_input_line(line);
        compact(&mut disk_line);
        assert_eq!(
            disk_line,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn get_checksum_test() {
        let line = "2333133121414131402";
        let mut disk_line = parse_input_line(line);
        compact(&mut disk_line);
        assert_eq!(get_checksum(&disk_line), 1928);
    }
}

pub mod part1 {
    use super::parse_input_line;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let input = parse_input_line(line);
            }
            "".to_string()
        }

        fn day() -> i32 {
            9
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
        #[test]
        fn test() {}
    }
}

pub mod part2 {
    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            "".to_string()
        }

        fn day() -> i32 {
            9
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
