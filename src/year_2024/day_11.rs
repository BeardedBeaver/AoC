use std::collections::HashMap;

type Stones = HashMap<u64, u64>;

fn parse_input(line: &str) -> Stones {
    let mut result = HashMap::new();
    for value in line
        .split(' ')
        .filter(|i| !i.is_empty())
        .map(|s| aoc::parse_or_panic::<u64>(s))
    {
        *result.entry(value).or_default() += 1;
    }

    result
}

fn get_number_of_digits(stone: u64) -> i32 {
    let mut result = 0;
    let mut stone = stone;
    while stone > 0 {
        stone /= 10;
        result += 1;
    }

    if result == 0 {
        result = 1
    }
    result
}

fn split_stone(stone: u64) -> (u64, u64) {
    let s = stone.to_string();
    (
        aoc::parse_or_panic(&s[0..s.len() / 2]),
        aoc::parse_or_panic(&s[s.len() / 2..]),
    )
}

fn count_stones(stones: &Stones) -> u64 {
    let mut result = 0;
    for (_, count) in stones.iter() {
        result += count;
    }
    result
}

fn blink(stones: &Stones) -> Stones {
    let mut result = Stones::new();

    for (stone, count) in stones.iter() {
        if *stone == 0 {
            *result.entry(1).or_default() += count;
        } else if get_number_of_digits(*stone) % 2 == 0 {
            let (s1, s2) = split_stone(*stone);

            *result.entry(s1).or_default() += count;
            *result.entry(s2).or_default() += count;
        } else {
            *result.entry(*stone * 2024).or_default() += count
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day_11::{count_stones, get_number_of_digits, split_stone};

    use super::{blink, parse_input};

    #[test]
    fn get_number_of_digits_test() {
        assert_eq!(get_number_of_digits(0), 1);
        assert_eq!(get_number_of_digits(1), 1);
        assert_eq!(get_number_of_digits(153), 3);
        assert_eq!(get_number_of_digits(50000), 5);
    }

    #[test]
    fn split_stone_test() {
        assert_eq!(split_stone(10), (1, 0));
        assert_eq!(split_stone(1000), (10, 0));
        assert_eq!(split_stone(123456), (123, 456));
    }

    #[test]
    fn blink_test() {
        let stones = parse_input("0 1 10 99 999");
        let stones = blink(&stones);

        assert_eq!(stones, parse_input("1 2024 1 0 9 9 2021976"))
    }

    #[test]
    fn blink_test_longer() {
        let mut stones = parse_input("125 17");
        for _ in 0..6 {
            stones = blink(&stones);
        }
        assert_eq!(
            stones,
            parse_input("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"),
        )
    }

    #[test]
    fn blink_test_25() {
        let mut stones = parse_input("125 17");
        for _ in 0..25 {
            stones = blink(&stones);
        }
        assert_eq!(count_stones(&stones), 55312);
    }
}

pub mod part1 {
    use super::{blink, count_stones, parse_input};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let mut stones = parse_input(line);
                for _ in 0..25 {
                    stones = blink(&stones);
                }
                return count_stones(&stones).to_string();
            }
            unreachable!()
        }

        fn day() -> i32 {
            11
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
    use super::{blink, count_stones, parse_input};

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
                let mut stones = parse_input(line);
                for _ in 0..75 {
                    stones = blink(&stones);
                }
                return count_stones(&stones).to_string();
            }
            unreachable!()
        }

        fn day() -> i32 {
            11
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2024
        }
    }
}
