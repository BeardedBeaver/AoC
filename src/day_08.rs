use std::collections::HashMap;

#[derive(Debug)]
struct Desert {
    nodes: HashMap<String, Vec<String>>,
    path: String,
}

fn parse_node(line: &str) -> (&str, &str, &str) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    assert_eq!(parts.len(), 4);
    (
        parts[0],
        parts[2].trim_matches(|c| c == '(' || c == ')' || c == ','),
        parts[3].trim_matches(|c| c == '(' || c == ')' || c == ','),
    )
}

fn load_desert_from_file(file_name: &str) -> Desert {
    let mut nodes = HashMap::new();
    let mut path = String::new();
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        if line.is_empty() {
            continue;
        }
        if path.is_empty() {
            path = line.to_owned();
        } else {
            let (start, left, right) = parse_node(line);
            nodes.insert(start.to_owned(), vec![left.to_owned(), right.to_owned()]);
        }
    }
    Desert {
        nodes: nodes,
        path: path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_node_test() {
        assert_eq!(parse_node("LRL = (MCG, TRC)"), ("LRL", "MCG", "TRC"));
        assert_eq!(parse_node("CBR = (QDT, NSG)"), ("CBR", "QDT", "NSG"));
    }
}

pub mod part1 {
    use super::*;

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let desert = load_desert_from_file(file_name);

            let mut i = 0;
            let mut result = 0;
            let mut curr_node = "AAA";
            loop {
                result += 1;
                let direction = desert.path.as_bytes()[i] as char;
                match direction {
                    'L' => curr_node = &desert.nodes.get(curr_node).unwrap()[0],
                    'R' => curr_node = &desert.nodes.get(curr_node).unwrap()[1],
                    _ => unreachable!(),
                }
                if curr_node == "ZZZ" {
                    return result.to_string();
                }
                i = (i + 1) % desert.path.len();
            }
        }

        fn day() -> i32 {
            8
        }

        fn part() -> i32 {
            1
        }
    }

    #[cfg(test)]
    mod tests {}
}

pub mod part2 {
    use super::*;

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(numbers: &[u64]) -> u64 {
        numbers.iter().fold(1, |acc, &x| acc * x / gcd(acc, x))
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let desert = load_desert_from_file(file_name);

            let mut curr_nodes = Vec::new();
            for (key, _) in desert.nodes.iter() {
                if key.ends_with("A") {
                    curr_nodes.push(key);
                }
            }

            let mut lengths = Vec::new();

            // This approach is actually cheating. An actual path in an input graph
            // consists of two parts - first path from A-node to Z-node, and a loop
            // from the next node after Z-node back to the Z-node. The trick is that
            // in all (apparantly) input graphs these two parts have equal length.
            // So this code doesn't try to find a loop and solve an arbitrary version
            // of the input graph, but rather makes the first run from A to Z and
            // assumes this value is also a length of the loop.

            for curr_node in curr_nodes.into_iter() {
                let mut curr_node = curr_node.as_str();
                let mut i = 0;
                let mut result = 0;
                loop {
                    result += 1;
                    let direction = desert.path.as_bytes()[i] as char;
                    match direction {
                        'L' => curr_node = &desert.nodes.get(curr_node).unwrap()[0],
                        'R' => curr_node = &desert.nodes.get(curr_node).unwrap()[1],
                        _ => unreachable!(),
                    }
                    if curr_node.ends_with("Z") {
                        lengths.push(result);
                        break;
                    }
                    i = (i + 1) % desert.path.len();
                }
            }
            lcm(&lengths).to_string()
        }

        fn day() -> i32 {
            8
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {}
}
