use std::collections::HashSet;

#[derive(Default)]
struct Card {
    id: u64,
    numbers: HashSet<u64>,
    winning_numbers: HashSet<u64>,
}

impl Card {
    pub fn num_wins(self: &Card) -> u64 {
        let mut result: u64 = 0;
        for num in &self.numbers {
            if self.winning_numbers.contains(&num) {
                result += 1;
            }
        }
        result
    }

    pub fn parse(line: &str) -> Card {
        let mut result = Card::default();

        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());

        let words: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        assert_eq!(2, words.len());
        result.id = words[1].parse::<u64>().unwrap();

        let parts: Vec<&str> = parts[1].split(" | ").collect();
        let numbers: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        let winning_numbers: Vec<&str> = parts[1].split_ascii_whitespace().collect();

        for num in numbers {
            result.numbers.insert(num.parse::<u64>().unwrap());
        }
        for num in winning_numbers {
            result.winning_numbers.insert(num.parse::<u64>().unwrap());
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_test() {
        let line = "Card   15: 42 5 78 89 | 42 99 1 89";
        let card = Card::parse(&line);
        assert_eq!(15, card.id);
        assert_eq!(2, card.num_wins());
    }
}

pub mod part1 {
    use super::*;

    pub fn get_score(wins: u64) -> u64 {
        if wins == 0 {
            return 0;
        }
        1 * u64::pow(2, wins as u32 - 1)
    }

    pub fn solve(file_name: &str) -> u64 {
        let mut result: u64 = 0;
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            let card = Card::parse(line);
            result += get_score(card.num_wins());
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_score_test() {
            assert_eq!(0, get_score(0));
            assert_eq!(1, get_score(1));
            assert_eq!(2, get_score(2));
            assert_eq!(4, get_score(3));
            assert_eq!(8, get_score(4));
        }
    }
}
