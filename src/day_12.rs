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

fn generate_all_possible_springs(pattern: &Vec<char>) -> LinkedList<Vec<char>> {
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

fn generate_possible_springs(pattern: &Vec<char>, damaged: &Vec<i32>) -> Vec<Vec<char>> {
    generate_all_possible_springs(&pattern)
        .iter()
        .filter(|config| {
            let pattern = get_broken_springs(config);
            pattern == *damaged
        })
        .map(|config| config.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_chars(s: &str) -> Vec<char> {
        s.as_bytes().iter().map(|c| *c as char).collect()
    }

    #[test]
    fn generate_all_possible_springs_test() {
        let pattern: Vec<char> = str_to_chars("??...#...#...");
        let springs = generate_all_possible_springs(&pattern);
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
        let configs = generate_possible_springs(&s.statuses, &s.damaged);
        return configs.len() as u64;
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

    fn get_combinations(configs: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut result = Vec::new();

        // well...
        for i1 in 0..configs.len() {
            for i2 in 0..configs.len() {
                for i3 in 0..configs.len() {
                    for i4 in 0..configs.len() {
                        for i5 in 0..configs.len() {
                            let mut case = Vec::new();
                            case.append(&mut configs[i1].clone());
                            case.push('?');
                            case.append(&mut configs[i2].clone());
                            case.push('?');
                            case.append(&mut configs[i3].clone());
                            case.push('?');
                            case.append(&mut configs[i4].clone());
                            case.push('?');
                            case.append(&mut configs[i5].clone());

                            result.push(case);
                        }
                    }
                }
            }
        }

        result
    }

    fn solve_spring(s: &Springs) -> u64 {
        let configs = generate_possible_springs(&s.statuses, &s.damaged);
        println!("{:?}", configs.len());
        let configs = get_combinations(&configs);
        println!("{:?}", configs.len());

        let damaged = s.damaged.repeat(5);

        let mut result = 0;
        for pattern in configs.iter() {
            let possible_configs = generate_possible_springs(pattern, &damaged);
            for c in possible_configs.iter() {
                let current_damaged = get_broken_springs(c);
                if current_damaged == damaged {
                    result += 1;
                }
            }
        }

        result
    }

    fn solve(springs: &Vec<Springs>) -> u64 {
        let mut result = 0;
        for s in springs.iter() {
            result += solve_spring(&s);
            break;
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
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_spring_test() {
            // assert_eq!(1, solve_spring(&Springs::parse("???.### 1,1,3")));
            assert_eq!(16384, solve_spring(&Springs::parse(".??..??...?##. 1,1,3")));
            assert_eq!(16, solve_spring(&Springs::parse("????.#...#... 4,1,1")));
        }
    }
}
