use std::usize;

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

fn rightmost_range(disk_line: &Vec<Option<i32>>, start_from: usize) -> Option<(i32, i32)> {
    if start_from >= disk_line.len() {
        return None;
    }

    let mut rp = start_from as i32;

    while disk_line[rp as usize].is_none() {
        rp -= 1;
        if rp < 0 {
            return None;
        }
    }

    let mut lp = rp;
    while lp >= 0 && disk_line[lp as usize] == disk_line[rp as usize] {
        lp -= 1;
    }

    Some((lp + 1, rp))
}

fn leftmost_empty_range(disk_line: &Vec<Option<i32>>, start_from: usize, size: usize) -> Option<(i32, i32)> {
    let mut lp = start_from as i32;
    while disk_line[lp as usize].is_some() {
        lp += 1;
        if lp >= disk_line.len() as i32 {
            return None;
        }
    }

    let mut rp = lp;
    for _ in 0..size {
        if disk_line[lp as usize] != disk_line[rp as usize] {
            return leftmost_empty_range(disk_line, rp as usize, size);
        }
        rp += 1;
        if rp >= disk_line.len() as i32 {
            return None;
        }
    }

    Some((lp, rp - 1))
}

fn swap_ranges(disk_line: &mut Vec<Option<i32>>, from: usize, to: usize, count: usize) {
    for i in 0..count {
        disk_line.swap(from + i, to + i);
    }
}

fn compact_no_fragmentation(disk_line: &mut Vec<Option<i32>>) {
    let mut lp = 0;
    let mut rp = disk_line.len() - 1;
    loop {
        let from = rightmost_range(disk_line, rp);
        if from.is_none() {
            return;
        }
        let from = from.unwrap();
        let to = leftmost_empty_range(disk_line, lp, (from.1 - from.0 + 1) as usize);
        if to.is_some() {
            let to = to.unwrap();
            if from.0 > to.0 {
                swap_ranges(disk_line, from.0 as usize, to.0 as usize, (to.1 - to.0 + 1) as usize);
            }
        }

        lp = 0;
        rp = (from.0 - 1) as usize;
    }
}

fn get_checksum(disk_line: &Vec<Option<i32>>) -> u64 {
    let mut result = 0;

    for (idx, maybe_value) in disk_line.iter().enumerate() {
        if maybe_value.is_none() {
            continue;
        }
        result += idx as u64 * maybe_value.unwrap() as u64;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day_09::{get_checksum, leftmost_empty_range};

    use super::{compact, compact_no_fragmentation, parse_input_line, rightmost_range};

    #[test]
    fn parse_input_line_test() {
        let input = "12345";
        let disk_line = parse_input_line(input);
        assert_eq!(
            disk_line,
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
        let disk_line = parse_input_line(input);
        assert_eq!(
            disk_line,
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
    fn rightmost_range_test() {
        let input = "2333133121414131402";
        let disk_line = parse_input_line(input);

        let range = rightmost_range(&disk_line, 41);
        assert_eq!(range, Some((40, 41)));

        let range = rightmost_range(&disk_line, 39);
        assert_eq!(range, Some((36, 39)));

        let range = rightmost_range(&disk_line, 35);
        assert_eq!(range, Some((32, 34)));

        let range = rightmost_range(&disk_line, 9);
        assert_eq!(range, Some((5, 7)));

        let range = rightmost_range(&disk_line, 2);
        assert_eq!(range, Some((0, 1)));
    }

    #[test]
    fn leftmost_empty_range_test() {
        let input = "2333133121414131402";
        let disk_line = parse_input_line(input);

        let range = leftmost_empty_range(&disk_line, 0, 2);
        assert_eq!(range, Some((2, 3)));

        let range = leftmost_empty_range(&disk_line, 0, 3);
        assert_eq!(range, Some((2, 4)));

        let range = leftmost_empty_range(&disk_line, 0, 4);
        assert_eq!(range, None);
    }

    #[test]
    fn compact_no_fragmentation_test() {
        let input = "2333133121414131402";
        let mut disk_line = parse_input_line(input);
        compact_no_fragmentation(&mut disk_line);
        assert_eq!(
            disk_line,
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(4),
                Some(4),
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                None,
                None,
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
                None,
                None,
                None,
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                None,
                None,
            ]
        )
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
    use super::{compact, get_checksum, parse_input_line};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let mut disk_line = parse_input_line(line);
                compact(&mut disk_line);
                result += get_checksum(&disk_line);
            }
            result.to_string()
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
    use super::{compact_no_fragmentation, get_checksum, parse_input_line};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let mut result = 0;
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let mut disk_line = parse_input_line(line);
                compact_no_fragmentation(&mut disk_line);
                result += get_checksum(&disk_line);
            }
            result.to_string()
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
