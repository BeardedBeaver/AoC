#[derive(Debug, PartialEq, Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_string(line: &str) -> Vec<Game> {
    let tries: Vec<_> = line.split(";").collect();
    let mut result = Vec::with_capacity(3);
    for t in tries {
        let words = t.split(", ");
        let mut game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };
        for word in words {
            let parts: Vec<&str> = word.trim().split(" ").collect();
            let value = parts[0].parse().unwrap();
            match parts[1] {
                "red" => game.red = value,
                "green" => game.green = value,
                "blue" => game.blue = value,
                _ => unreachable!(),
            }
        }
        result.push(game);
    }
    result
}

fn get_game_index(line: &str) -> u64 {
    let words: Vec<&str> = line.split(" ").collect();
    assert_eq!(words.len(), 2);
    return words[1].parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_game_index_test() {
        assert_eq!(4, get_game_index("Game 4"));
    }

    #[test]
    fn get_value_from_line_test() {
        assert_eq!(
            vec![Game {
                red: 2,
                green: 1,
                blue: 3
            }],
            parse_string("1 green, 2 red, 3 blue")
        );
        assert_eq!(
            vec![Game {
                red: 4,
                green: 7,
                blue: 0
            }],
            parse_string("4 red, 7 green")
        );
    }
}

pub mod part1 {
    use super::*;

    fn is_good_game(line: &str) -> bool {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let games = parse_string(line);
        for game in games {
            max_red = std::cmp::max(max_red, game.red);
            max_green = std::cmp::max(max_green, game.green);
            max_blue = std::cmp::max(max_blue, game.blue);
        }
        return max_red <= 12 && max_green <= 13 && max_blue <= 14;
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut result: u64 = 0;
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                let words: Vec<&str> = line.split(":").collect();
                assert_eq!(words.len(), 2);
                let game_index = get_game_index(words[0]);
                if is_good_game(words[1]) {
                    result += game_index;
                }
            }
            result.to_string()
        }

        fn day() -> i32 {
            2
        }

        fn part() -> i32 {
            1
        }
    }
}

pub mod part2 {
    use super::*;

    fn get_powers(line: &str) -> u64 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let games = parse_string(line);
        for game in games {
            max_red = std::cmp::max(max_red, game.red);
            max_green = std::cmp::max(max_green, game.green);
            max_blue = std::cmp::max(max_blue, game.blue);
        }
        return max_red as u64 * max_green as u64 * max_blue as u64;
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut result: u64 = 0;
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                let words: Vec<&str> = line.split(":").collect();
                assert_eq!(words.len(), 2);
                result += get_powers(words[1]);
            }
            result.to_string()
        }

        fn day() -> i32 {
            2
        }

        fn part() -> i32 {
            2
        }
    }
}
