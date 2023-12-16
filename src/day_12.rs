use std::collections::LinkedList;

#[derive(Debug, PartialEq, Clone)]
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

fn get_damaged_springs(springs: &Vec<char>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current_damaged_count = 0;
    for c in springs.iter() {
        if *c == '?' {
            break;
        }
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

fn is_possible(pattern: &Vec<char>, damaged: &Vec<i32>) -> bool {
    let current_damaged = get_damaged_springs(&pattern);

    if current_damaged.len() < 2 {
        return true;
    }

    if current_damaged.len() > damaged.len() {
        return false;
    }

    for i in 0..current_damaged.len() - 1 {
        if i >= damaged.len() {
            return false;
        }
        if current_damaged[i] > damaged[i] {
            return false;
        }
    }

    return true;
}

fn generate_filtered_possible_springs(pattern: &Vec<char>, damaged: &Vec<i32>) -> LinkedList<Vec<char>> {
    let mut result = LinkedList::new();
    result.push_back(pattern.to_owned());
    while result.front().unwrap().iter().any(|c| *c == '?') {
        let current = result.pop_front().unwrap();
        let index = current.iter().position(|c| *c == '?').unwrap();

        let mut lhs = current.clone();
        lhs[index] = '.';

        let mut rhs = current.clone();
        rhs[index] = '#';

        if is_possible(&lhs, &damaged) {
            result.push_back(lhs);
        }
        if is_possible(&rhs, &damaged) {
            result.push_back(rhs);
        }
    }
    result
}

fn generate_possible_springs(pattern: &Vec<char>, damaged: &Vec<i32>) -> Vec<Vec<char>> {
    generate_filtered_possible_springs(&pattern, &damaged)
        .iter()
        .filter(|config| {
            let pattern = get_damaged_springs(config);
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
    fn get_damaged_springs_test() {
        assert_eq!(vec![3], get_damaged_springs(&str_to_chars("...###")));
        assert_eq!(vec![1, 1, 3], get_damaged_springs(&str_to_chars("#.#.###")));
        assert_eq!(vec![1, 3, 1, 6], get_damaged_springs(&str_to_chars(".#.###.#.######")));
        assert_eq!(vec![4, 1, 1], get_damaged_springs(&str_to_chars("####.#...#...")));
        assert_eq!(vec![1, 6, 5], get_damaged_springs(&str_to_chars("#....######..#####.")));
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

    fn fold(s: &mut Springs, fold_factor: usize) {
        let mut statuses = Vec::new();
        for _ in 0..fold_factor - 1 {
            statuses.append(&mut s.statuses.clone());
            statuses.push('?');
        }
        statuses.append(&mut s.statuses);
        s.statuses = statuses;

        s.damaged = s.damaged.repeat(fold_factor);
    }

    fn solve_spring(s: &Springs) -> u64 {
        let c1 = generate_possible_springs(&s.statuses, &s.damaged);
        let f1 = c1.len();

        let mut s2 = s.clone();
        fold(&mut s2, 2);
        let c2 = generate_possible_springs(&s2.statuses, &s2.damaged);
        let f2 = c2.len();

        let factor = f2 / f1;
        println!("{f1}, {f2}, {factor}");
        return (f2 * factor * factor * factor) as u64;
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
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fold_test() {
            let mut s = Springs::parse("???.### 1,1,3");
            fold(&mut s, 5);
            assert_eq!(
                Springs::parse("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"),
                s
            );

            let mut s = Springs::parse(".??..??...?##. 1,1,3");
            fold(&mut s, 5);
            assert_eq!(
                Springs::parse(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"),
                s
            );
        }

        #[test]
        fn solve_spring_test() {
            assert_eq!(1, solve_spring(&Springs::parse("???.### 1,1,3")));
            assert_eq!(16384, solve_spring(&Springs::parse(".??..??...?##. 1,1,3")));
            assert_eq!(16, solve_spring(&Springs::parse("????.#...#... 4,1,1")));
            assert_eq!(506250, solve_spring(&Springs::parse("?###???????? 3,2,1")));
        }
    }
}
