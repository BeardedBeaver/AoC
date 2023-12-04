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

pub mod part2 {
    use super::*;

    struct Pile {
        cards: Vec<u64>,
    }

    impl Pile {
        pub fn new() -> Pile {
            Pile { cards: vec![1] }
        }

        pub fn put(self: &mut Pile, n: u64) {
            for i in 1..=n {
                match self.cards.len().cmp(&(i as usize)) {
                    std::cmp::Ordering::Less => unreachable!(),
                    std::cmp::Ordering::Equal => self.cards.push(1),
                    std::cmp::Ordering::Greater => self.cards[i as usize] += 1,
                }
            }
        }

        pub fn get(self: &mut Pile) -> bool {
            self.cards[0] -= 1;
            if self.cards[0] == 0 {
                self.cards.remove(0);
                return true;
            }
            false
        }

        pub fn is_empty(self: &Pile) -> bool {
            for c in &self.cards {
                if *c > 0 {
                    return false;
                }
            }
            true
        }
    }

    fn count_cards(cards: &Vec<Card>) -> u64 {
        let mut pile = Pile::new();
        let mut current_index: usize = 0;
        let mut result = 0;

        while !pile.is_empty() {
            let card = &cards[current_index];
            let wins = card.num_wins();
            pile.put(wins);
            if pile.get() {
                current_index += 1;
            }
            result += 1;
        }

        result
    }

    pub fn solve(file_name: &str) -> u64 {
        let mut cards: Vec<Card> = Vec::new();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            let card = Card::parse(line);
            cards.push(card);
        }
        count_cards(&cards)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn pile_test() {
            let mut pile = Pile::new();
            assert!(!pile.is_empty());

            pile.put(3);

            assert_eq!(vec![1, 1, 1, 1], pile.cards);
            assert!(pile.get());

            assert_eq!(vec![1, 1, 1], pile.cards);
            
            pile.put(3);
            assert_eq!(vec![1, 2, 2, 1], pile.cards);

            pile.put(4);
            assert_eq!(vec![1, 3, 3, 2, 1], pile.cards);

            assert!(pile.get());
            assert!(!pile.get());
            assert_eq!(vec![2, 3, 2, 1], pile.cards);

            for _ in 0..5 {
                pile.get();
            }
            assert_eq!(vec![2, 1], pile.cards);
            assert!(!pile.is_empty());

            pile.put(1);
            assert_eq!(vec![2, 2], pile.cards);

            for _ in 0..4 {
                pile.get();
            }
            assert!(pile.cards.is_empty());
            assert!(pile.is_empty());

        }

        #[test]
        fn count_cards_test() {
            let mut cards = Vec::new();
            cards.push(Card::parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            ));
            cards.push(Card::parse(
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            ));
            cards.push(Card::parse(
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            ));
            cards.push(Card::parse(
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            ));
            cards.push(Card::parse(
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            ));
            cards.push(Card::parse(
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ));

            assert_eq!(30, count_cards(&cards));
        }
    }
}
