use std::ops::Add;

fn parse_input(file_name: &str) -> Vec<String> {
    let mut s = String::default();

    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        s = s.add(line);
    }

    s.split(',').map(|s| s.to_string()).collect::<Vec<String>>()
}

fn get_hash(s: &str) -> usize {
    let mut result = 0;

    for c in s.chars() {
        result += c as u64;
        result *= 17;
        result %= 256;
    }

    result as usize
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

struct HashMap {
    buckets: Vec<Vec<Lens>>,
}

impl Default for HashMap {
    fn default() -> HashMap {
        let buckets = vec![vec![]; 256];
        HashMap { buckets: buckets }
    }
}

pub mod part1 {
    use super::*;

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let strings = parse_input(file_name);
            let mut result = 0;

            for s in strings.iter() {
                result += get_hash(s);
            }

            result.to_string()
        }

        fn day() -> i32 {
            15
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_hash_test() {
            assert_eq!(52, get_hash("HASH"));

            assert_eq!(30, get_hash("rn=1"));
            assert_eq!(253, get_hash("cm-"));
            assert_eq!(97, get_hash("qp=3"));
        }
    }
}

pub mod part2 {
    use super::*;

    #[derive(Debug)]
    enum Action {
        Insert(Lens),
        Remove(String),
    }

    fn parse_action(s: &str) -> Action {
        if s.contains("=") {
            let parts = s.split("=").collect::<Vec<&str>>();
            return Action::Insert(Lens {
                label: parts[0].to_owned(),
                focal_length: parts[1].parse().unwrap(),
            });
        }
        if s.contains("-") {
            let parts = s.split("-").collect::<Vec<&str>>();
            return Action::Remove(parts[0].to_string());
        }
        unreachable!();
    }

    fn do_action(map: &mut HashMap, action: Action) {
        match action {
            Action::Insert(lens) => {
                let hash = get_hash(&lens.label);
                assert!(hash < 256);
                if let Some(existing) = map.buckets[hash]
                    .iter_mut()
                    .find(|existing| *existing.label == lens.label)
                {
                    *existing = lens;
                } else {
                    map.buckets[hash].push(lens);
                }
            }
            Action::Remove(label) => {
                let hash = get_hash(&label);
                assert!(hash < 256);
                map.buckets[hash].retain(|lens| lens.label != label);
            }
        }
    }

    fn get_score(map: &HashMap) -> u64 {
        let mut result: usize = 0;
        for (i, bucket) in map.buckets.iter().enumerate() {
            for (j, lens) in bucket.iter().enumerate() {
                let lens_value = lens.focal_length * (j + 1) * (i + 1);
                result += lens_value;
            }
        }
        result as u64
    }

    fn solve(actions: &Vec<String>) -> u64 {
        let mut map = HashMap::default();

        for a in actions.iter() {
            let action = parse_action(a);
            do_action(&mut map, action);
        }

        get_score(&map)
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let strings = parse_input(file_name);
            solve(&strings).to_string()
        }

        fn day() -> i32 {
            15
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_test() {
            let actions = vec![
                "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            assert_eq!(145, solve(&actions));
        }
    }
}
