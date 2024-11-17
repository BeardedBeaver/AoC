use std::collections::HashSet;

#[derive(Default)]
struct Card {
    id: u64,
    numbers: HashSet<u64>,
    winning: HashSet<u64>,
}

impl Card {
    pub fn num_wins(self: &Card) -> u64 {
        self.numbers.intersection(&self.winning).count() as u64
    }

    pub fn parse(line: &str) -> Card {
        let mut result = Card::default();

        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());

        let words: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        assert_eq!(2, words.len());
        result.id = words[1].parse::<u64>().unwrap();

        let parts: Vec<&str> = parts[1].split(" | ").collect();
        let winning: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        let numbers: Vec<&str> = parts[1].split_ascii_whitespace().collect();

        result.winning = winning.iter().map(|x| x.parse::<u64>().unwrap()).collect();
        result.numbers = numbers.iter().map(|x| x.parse::<u64>().unwrap()).collect();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_test() {
        let line = "Card   15: 42 5 78 89 | 42 99  1 89";
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

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut result: u64 = 0;
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                let card = Card::parse(line);
                result += get_score(card.num_wins());
            }
            result.to_string()
        }

        fn day() -> i32 {
            4
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2023
        }
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

pub mod part2 {
    use super::*;

    fn count_cards(cards: &Vec<Card>) -> u64 {
        let mut pile = vec![1; cards.len()];

        for (i, card) in cards.iter().enumerate() {
            let wins = card.num_wins();
            for j in 0..wins {
                let index = i + j as usize + 1;
                if index >= cards.len() {
                    break;
                }
                pile[index] += pile[i];
            }
        }

        pile.iter().sum()
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut cards: Vec<Card> = Vec::new();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                let card = Card::parse(line);
                cards.push(card);
            }
            count_cards(&cards).to_string()
        }

        fn day() -> i32 {
            4
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn count_cards_test() {
            // just one card
            {
                let mut cards = Vec::new();
                cards.push(Card::parse("Card 1: 1 2 3 | 1 2 3"));
                assert_eq!(1, count_cards(&cards));
            }

            // example case from the puzzle
            {
                let mut cards = Vec::new();
                cards.push(Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
                cards.push(Card::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"));
                cards.push(Card::parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"));

                assert_eq!(7, count_cards(&cards));

                cards.push(Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
                cards.push(Card::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));

                assert_eq!(29, count_cards(&cards));

                cards.push(Card::parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));

                assert_eq!(30, count_cards(&cards));
            }
        }
    }
}
