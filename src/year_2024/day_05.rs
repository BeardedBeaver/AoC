use std::collections::HashMap;

struct ValidationRule {
    before: i32,
    after: i32,
}

fn is_rule_satisfied(ordering: &HashMap<i32, i32>, rule: &ValidationRule) -> bool {
    let maybe_before = ordering.get(&rule.before);
    let maybe_after = ordering.get(&rule.after);
    if maybe_before.is_none() || maybe_after.is_none() {
        return true;
    }
    maybe_before.unwrap() < maybe_after.unwrap()
}

fn is_valid(pages: &Vec<i32>, rules: &Vec<ValidationRule>) -> bool {
    // prepare page ordering
    let mut ordering: HashMap<i32, i32> = HashMap::with_capacity(pages.len());
    for (idx, page_number) in pages.iter().enumerate() {
        ordering.insert(*page_number, idx as i32);
    }

    // check if all rules are satisfied
    for rule in rules.iter() {
        if !is_rule_satisfied(&ordering, rule) {
            return false;
        }
    }

    true
}

fn parse_input(input_file_name: &str) -> (Vec<ValidationRule>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut pages = Vec::new();
    let mut reading_rules = true;
    for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
        if reading_rules {
            if line.is_empty() {
                reading_rules = false;
            } else {
                let parts: Vec<&str> = line.split('|').collect();
                rules.push(ValidationRule {
                    before: aoc::parse_or_panic(parts[0]),
                    after: aoc::parse_or_panic(parts[1]),
                });
            }
        } else {
            pages.push(line.split(',').map(|s| aoc::parse_or_panic::<i32>(s)).collect());
        }
    }

    (rules, pages)
}

#[allow(dead_code)] // used in tests
fn get_test_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule { before: 47, after: 53 },
        ValidationRule { before: 97, after: 13 },
        ValidationRule { before: 97, after: 61 },
        ValidationRule { before: 97, after: 47 },
        ValidationRule { before: 75, after: 29 },
        ValidationRule { before: 61, after: 13 },
        ValidationRule { before: 75, after: 53 },
        ValidationRule { before: 29, after: 13 },
        ValidationRule { before: 97, after: 29 },
        ValidationRule { before: 53, after: 29 },
        ValidationRule { before: 61, after: 53 },
        ValidationRule { before: 97, after: 53 },
        ValidationRule { before: 61, after: 29 },
        ValidationRule { before: 47, after: 13 },
        ValidationRule { before: 75, after: 47 },
        ValidationRule { before: 97, after: 75 },
        ValidationRule { before: 47, after: 61 },
        ValidationRule { before: 75, after: 61 },
        ValidationRule { before: 47, after: 29 },
        ValidationRule { before: 75, after: 13 },
        ValidationRule { before: 53, after: 13 },
    ]
}

pub mod part1 {
    use super::{is_valid, parse_input, ValidationRule};

    fn solve(pages: &Vec<Vec<i32>>, rules: &Vec<ValidationRule>) -> i32 {
        let mut result = 0;
        for page_set in pages {
            if is_valid(&page_set, &rules) {
                result += page_set[page_set.len() / 2];
            }
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (rules, pages) = parse_input(input_file_name);
            solve(&pages, &rules).to_string()
        }

        fn day() -> i32 {
            5
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
        use crate::day_05::{get_test_rules, is_valid, part1::solve};

        #[test]
        fn is_valid_test() {
            let rules = get_test_rules();

            let pages = vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ];

            assert_eq!(is_valid(&pages[0], &rules), true);
            assert_eq!(is_valid(&pages[1], &rules), true);
            assert_eq!(is_valid(&pages[2], &rules), true);
            assert_eq!(is_valid(&pages[3], &rules), false);
            assert_eq!(is_valid(&pages[4], &rules), false);
            assert_eq!(is_valid(&pages[5], &rules), false);

            assert_eq!(solve(&pages, &rules), 143);
        }
    }
}

pub mod part2 {
    use std::cmp::Ordering;

    use super::{is_valid, parse_input, ValidationRule};

    fn fix_ordering(pages: &Vec<i32>, rules: &Vec<ValidationRule>) -> Vec<i32> {
        let mut result = pages.clone();

        result.sort_by(|left, right| {
            for rule in rules.iter() {
                if rule.before == *left && rule.after == *right {
                    return Ordering::Less;
                } else if rule.before == *right && rule.after == *left {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        });

        result
    }

    fn solve(pages: &Vec<Vec<i32>>, rules: &Vec<ValidationRule>) -> i32 {
        let mut result = 0;
        for page_set in pages {
            if !is_valid(&page_set, &rules) {
                let fixed = fix_ordering(&page_set, &rules);
                result += fixed[fixed.len() / 2];
            }
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let (rules, pages) = parse_input(input_file_name);
            solve(&pages, &rules).to_string()
        }

        fn day() -> i32 {
            5
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
        use crate::day_05::{
            get_test_rules,
            part2::{fix_ordering, solve},
        };

        #[test]
        fn test_fix_ordering() {
            let rules = get_test_rules();
            assert_eq!(
                fix_ordering(&vec![75, 97, 47, 61, 53], &rules),
                vec![97, 75, 47, 61, 53]
            );

            assert_eq!(fix_ordering(&vec![61, 13, 29], &rules), vec![61, 29, 13]);

            assert_eq!(
                fix_ordering(&vec![97, 13, 75, 29, 47], &rules),
                vec![97, 75, 47, 29, 13]
            );
        }

        #[test]
        fn test_solve() {
            let rules = get_test_rules();

            let pages = vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ];

            assert_eq!(solve(&pages, &rules), 123);
        }
    }
}
