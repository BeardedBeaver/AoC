use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
struct Springs {
    statuses: Vec<char>,
    damaged: Vec<i32>, // the size of each contiguous group of damaged (#) springs
}

impl Springs {
    fn parse(line: &str) -> Springs {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(2, parts.len());
        let damaged = parts[1].split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        Springs {
            statuses: parts[0].as_bytes().iter().map(|c| *c as char).collect(),
            damaged: damaged,
        }
    }
}

fn get_broken_springs(springs: &Vec<char>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current_damaged_count = 0;
    for c in springs.iter() {
        if *c == '.' && current_damaged_count > 0 {
            result.push(current_damaged_count);
            current_damaged_count = 0;
        } else if *c == '#' {
            current_damaged_count += 1
        }
    }
    if current_damaged_count > 0 {
        result.push(current_damaged_count);
    }
    result
}

fn generate_possible_springs(pattern: &Vec<char>) -> LinkedList<Vec<char>> {
    let mut result = LinkedList::new();
    result.push_back(pattern.to_owned());
    while result.front().unwrap().iter().any(|c| *c == '?') {
        let current = result.pop_front().unwrap();
        let index = current.iter().position(|c| *c == '?').unwrap();

        let mut lhs = current.clone();
        lhs[index] = '.';

        let mut rhs = current.clone();
        rhs[index] = '#';

        result.push_back(lhs);
        result.push_back(rhs);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_chars(s: &str) -> Vec<char> {
        s.as_bytes().iter().map(|c| *c as char).collect()
    }

    #[test]
    fn generate_possible_springs_test() {
        let pattern: Vec<char> = str_to_chars("??...#...#...");
        let springs = generate_possible_springs(&pattern);
        assert_eq!(4, springs.len());
        assert!(springs.contains(&str_to_chars(".....#...#...")));
        assert!(springs.contains(&str_to_chars("#....#...#...")));
        assert!(springs.contains(&str_to_chars(".#...#...#...")));
        assert!(springs.contains(&str_to_chars("##...#...#...")));
    }

    #[test]
    fn get_broken_springs_test() {
        assert_eq!(vec![3], get_broken_springs(&str_to_chars("...###")));
        assert_eq!(vec![1, 1, 3], get_broken_springs(&str_to_chars("#.#.###")));
        assert_eq!(vec![1, 3, 1, 6], get_broken_springs(&str_to_chars(".#.###.#.######")));
        assert_eq!(vec![4, 1, 1], get_broken_springs(&str_to_chars("####.#...#...")));
        assert_eq!(vec![1, 6, 5], get_broken_springs(&str_to_chars("#....######..#####.")));
    }
}

pub mod part1 {
    use super::*;

    fn solve_spring(s: &Springs) -> u64 {
        let mut result = 0;
        let configs = generate_possible_springs(&s.statuses);
        for config in configs.iter() {
            let pattern = get_broken_springs(&config);
            if pattern == s.damaged {
                result += 1;
            }
        }
        result
    }

    fn solve(springs: &Vec<Springs>) -> u64 {
        let mut result = 0;
        for s in springs.iter() {
            result += solve_spring(&s);
        }
        result
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut springs = Vec::new();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                let s = Springs::parse(line);
                springs.push(s);
            }
            solve(&springs).to_string()
        }

        fn day() -> i32 {
            12
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_spring_test() {
            let s = Springs::parse("???.### 1,1,3");
            assert_eq!(1, solve_spring(&s));

            let s = Springs::parse(".??..??...?##. 1,1,3");
            assert_eq!(4, solve_spring(&s));

            let s = Springs::parse("????.######..#####. 1,6,5");
            assert_eq!(4, solve_spring(&s));

            let s = Springs::parse("?###???????? 3,2,1");
            assert_eq!(10, solve_spring(&s));
        }
    }
}

pub mod part2 {
    use super::*;

    fn fold(s: &mut Springs) {
        let mut statuses = Vec::new();
        for _ in 0..4 {
            statuses.append(&mut s.statuses.clone());
            statuses.push('?');
        }
        statuses.append(&mut s.statuses);
        s.statuses = statuses;

        s.damaged = s.damaged.repeat(5);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fold_test() {
            let mut s = Springs::parse("???.### 1,1,3");
            fold(&mut s);
            assert_eq!(
                Springs::parse("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"),
                s
            );

            let mut s = Springs::parse(".??..??...?##. 1,1,3");
            fold(&mut s);
            assert_eq!(
                Springs::parse(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"),
                s
            );
        }
    }
}
