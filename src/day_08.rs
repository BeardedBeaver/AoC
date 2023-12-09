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

    pub fn solve(file_name: &str) -> u64 {
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
                return result;
            }
            i = (i + 1) % desert.path.len();
        }
    }

    #[cfg(test)]
    mod tests {}
}
