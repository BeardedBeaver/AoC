pub mod part1 {
    const XMAS_INDEX_PATTERNS: [&[(i32, i32)]; 8] = [
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
        &[(0, 0), (0, -1), (0, -2), (0, -3)],
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        &[(0, 0), (1, 1), (2, 2), (3, 3)],
        &[(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        &[(0, 0), (1, -1), (2, -2), (3, -3)],
        &[(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    ];

    const XMAS: &str = "XMAS";

    fn is_xmas(input: &[String], row: i32, col: i32, pattern: &[(i32, i32)]) -> bool {
        for (idx, c) in XMAS.chars().enumerate() {
            let i = row + pattern[idx].0;
            let j = col + pattern[idx].1;
            if i < 0 || i >= input.len() as i32 || j < 0 || j >= input[0].len() as i32 {
                return false;
            }
            let i = i as usize;
            let j = j as usize;
            if input[i].as_bytes()[j] != c as u8 {
                return false;
            }
        }
        true
    }

    fn count_xmas_around_pos(input: &[String], row: i32, col: i32) -> i32 {
        let mut result = 0;
        for pattern in XMAS_INDEX_PATTERNS {
            if is_xmas(input, row, col, pattern) {
                result += 1;
            }
        }
        result
    }

    fn count_xmas(input: &[String]) -> i32 {
        let mut result = 0;
        for (i, row) in input.iter().enumerate() {
            for (j, char) in row.chars().enumerate() {
                if char == 'X' {
                    result += count_xmas_around_pos(&input, i as i32, j as i32);
                }
            }
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let lines: Vec<String> = std::fs::read_to_string(input_file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
            count_xmas(&lines).to_string()
        }

        fn day() -> i32 {
            4
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
        fn count_xmas_test() {
            let input = vec![
                "MMMSXXMASM".to_string(),
                "MSAMXMSMSA".to_string(),
                "AMXSXMAAMM".to_string(),
                "MSAMASMSMX".to_string(),
                "XMASAMXAMM".to_string(),
                "XXAMMXXAMA".to_string(),
                "SMSMSASXSS".to_string(),
                "SAXAMASAAA".to_string(),
                "MAMMMXMMMM".to_string(),
                "MXMXAXMASX".to_string(),
            ];
            assert_eq!(super::count_xmas(&input), 18);
        }
    }
}

pub mod part2 {
    const X_MAS_INDEX_PATTERNS: [&[(i32, i32)]; 4] = [
        &[(-1, -1), (0, 0), (1, 1)],
        &[(1, 1), (0, 0), (-1, -1)],
        &[(1, -1), (0, 0), (-1, 1)],
        &[(-1, 1), (0, 0), (1, -1)],
    ];

    const MAS: &str = "MAS";

    fn is_mas(input: &[String], row: i32, col: i32, pattern: &[(i32, i32)]) -> bool {
        for (idx, c) in MAS.chars().enumerate() {
            let i = row + pattern[idx].0;
            let j = col + pattern[idx].1;
            if i < 0 || i >= input.len() as i32 || j < 0 || j >= input[0].len() as i32 {
                return false;
            }
            let i = i as usize;
            let j = j as usize;
            if input[i].as_bytes()[j] != c as u8 {
                return false;
            }
        }
        true
    }

    fn count_x_mas_around_pos(input: &[String], row: i32, col: i32) -> i32 {
        let mut result = 0;
        for pattern in X_MAS_INDEX_PATTERNS {
            if is_mas(input, row, col, pattern) {
                result += 1;
            }
        }
        result
    }

    fn is_x_mas(input: &[String], row: i32, col: i32) -> bool {
        count_x_mas_around_pos(&input, row, col) == 2
    }

    fn count_x_mas(input: &[String]) -> i32 {
        let mut result = 0;
        for (i, row) in input.iter().enumerate() {
            for (j, char) in row.chars().enumerate() {
                if char == 'A' && is_x_mas(&input, i as i32, j as i32) {
                    result += 1;
                }
            }
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(input_file_name: &str) -> String {
            let lines: Vec<String> = std::fs::read_to_string(input_file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
            count_x_mas(&lines).to_string()
        }

        fn day() -> i32 {
            4
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
        fn count_x_mas_test() {
            let input = vec![
                "MMMSXXMASM".to_string(),
                "MSAMXMSMSA".to_string(),
                "AMXSXMAAMM".to_string(),
                "MSAMASMSMX".to_string(),
                "XMASAMXAMM".to_string(),
                "XXAMMXXAMA".to_string(),
                "SMSMSASXSS".to_string(),
                "SAXAMASAAA".to_string(),
                "MAMMMXMMMM".to_string(),
                "MXMXAXMASX".to_string(),
            ];
            assert_eq!(super::count_x_mas(&input), 9);
        }
    }
}
