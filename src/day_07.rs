use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: u64,
}

impl Hand {
    pub fn from_string(s: &str) -> Hand {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        Hand {
            cards: parts[0].to_owned(),
            bid: parts[1].parse::<u64>().unwrap(),
        }
    }

    // 6 - Five of a kind
    // 5 - Four of a kind
    // 4 - Full house (3 + 2)
    // 3 - Three of a kind
    // 2 - Two pair
    // 1 - One pair
    // 0 - High card
    pub fn strength(self: &Hand) -> u64 {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for c in self.cards.chars() {
            *counts.entry(c).or_insert(0) += 1
        }
        let scores = freqs_to_tuple(&counts);
        match scores {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        }
    }
}

fn freqs_to_tuple(freqs: &HashMap<char, u64>) -> (u64, u64) {
    let mut frequencies: Vec<u64> = freqs.values().cloned().collect();
    frequencies.sort_by_key(|v| std::cmp::Reverse(*v));
    (*frequencies.get(0).unwrap_or(&0), *frequencies.get(1).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hand_test() {
        assert_eq!(
            Hand {
                cards: "32T3K".to_owned(),
                bid: 765
            },
            Hand::from_string("32T3K 765")
        );
        assert_eq!(
            Hand {
                cards: "T55J5".to_owned(),
                bid: 684
            },
            Hand::from_string("T55J5 684")
        );
    }

    #[test]
    fn strength_test() {
        assert_eq!(
            Hand {
                cards: "32T3K".to_owned(),
                bid: 765
            }
            .strength(),
            1
        );
        assert_eq!(
            Hand {
                cards: "4TT64".to_owned(),
                bid: 765
            }
            .strength(),
            2
        );
        assert_eq!(
            Hand {
                cards: "33333".to_owned(),
                bid: 765
            }
            .strength(),
            6
        );
    }
}

fn solve_hands(hands: &Vec<Hand>) -> u64 {
    let mut result: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i as u64 + 1);
    }

    result
}

pub mod part1 {
    use super::*;

    fn get_card_score(card: char) -> u64 {
        match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => 0,
        }
    }

    fn cmp(lhs: &Hand, rhs: &Hand) -> std::cmp::Ordering {
        let result = lhs.strength().cmp(&rhs.strength());
        match result {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let lhs_score = get_card_score(lhs.cards.as_bytes()[i] as char);
                    let rhs_score = get_card_score(rhs.cards.as_bytes()[i] as char);
                    if lhs_score == rhs_score {
                        continue;
                    }
                    let result = lhs_score.cmp(&rhs_score);
                    return result;
                }
                std::cmp::Ordering::Equal
            }
            _ => result,
        }
    }

    pub fn solve(file_name: &str) -> u64 {
        let mut hands = Vec::new();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            hands.push(Hand::from_string(line));
        }
        hands.sort_unstable_by(|lhs, rhs| cmp(&lhs, &rhs));
        solve_hands(&hands)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn hand_cmp_test() {
            assert_eq!(
                cmp(&Hand::from_string("32T4K 765"), &Hand::from_string("T55J5 684")),
                std::cmp::Ordering::Less
            );
            assert_eq!(
                cmp(&Hand::from_string("32T3K 765"), &Hand::from_string("T55J5 684")),
                std::cmp::Ordering::Less
            );
            assert_eq!(
                cmp(&Hand::from_string("KK677 28"), &Hand::from_string("KTJJT 220")),
                std::cmp::Ordering::Greater
            );
            assert_eq!(
                cmp(&Hand::from_string("KK677 28"), &Hand::from_string("QQQJA 483")),
                std::cmp::Ordering::Less
            );
            assert_eq!(
                cmp(&Hand::from_string("KK677 28"), &Hand::from_string("KK677 218")),
                std::cmp::Ordering::Equal
            );
        }

        #[test]
        fn solve_test() {
            // this is already sorted because solve_hands accepts a sorted vector
            let hands = vec![
                Hand::from_string("32T3K 765"),
                Hand::from_string("KTJJT 220"),
                Hand::from_string("KK677 28"),
                Hand::from_string("T55J5 684"),
                Hand::from_string("QQQJA 483"),
            ];
            assert_eq!(6440, solve_hands(&hands));
        }
    }
}

pub mod part2 {
    use super::*;

    fn get_card_score(card: char) -> u64 {
        match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => 0,
        }
    }

    fn preprocess_hand(hand: &str) -> String {
        // create a frequency map of all cards except jokers
        let mut counts: HashMap<char, u64> = HashMap::new();
        for c in hand.chars() {
            if c == 'J' {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1
        }

        // maps frequency to a list of characters
        let mut freq_map: HashMap<u64, Vec<char>> = HashMap::new();
        for (char, freq) in counts.iter() {
            freq_map.entry(*freq).or_insert(vec![]).push(*char);
        }

        // if the hand is all jokers replace them with aces and return
        if freq_map.is_empty() {
            return "AAAAA".to_owned();
        }

        let mut result = hand.to_owned();

        // loop through all frequencies from highest to lowest and get
        // the highest card from the first non-empty stack
        for i in (1..=5).rev() {
            if let Some(cards) = freq_map.get(&i) {
                if cards.is_empty() {
                    continue;
                }
                let card = get_highest_card(&cards);
                result = result.replace('J', &card.to_string());
                return result;
            }
        }
        unreachable!();
    }

    fn cmp(lhs: &Hand, rhs: &Hand) -> std::cmp::Ordering {
        let joked_lhs = Hand {
            cards: preprocess_hand(&lhs.cards),
            bid: 0,
        };
        let joked_rhs = Hand {
            cards: preprocess_hand(&rhs.cards),
            bid: 0,
        };

        // first we compare cards after replacing joker, then if it's a tie, fallback to
        // comparing the original cards
        let result = joked_lhs.strength().cmp(&joked_rhs.strength());
        match result {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let lhs_score = get_card_score(lhs.cards.as_bytes()[i] as char);
                    let rhs_score = get_card_score(rhs.cards.as_bytes()[i] as char);
                    if lhs_score == rhs_score {
                        continue;
                    }
                    let result = lhs_score.cmp(&rhs_score);
                    return result;
                }
                std::cmp::Ordering::Equal
            }
            _ => result,
        }
    }

    fn get_highest_card(cards: &Vec<char>) -> char {
        if cards.len() == 1 {
            return cards[0];
        }
        // screw it, I'm going dirty
        for c in "AKQT98765432J".as_bytes() {
            let c = *c as char;
            if cards.contains(&c) {
                return c;
            }
        }
        unreachable!();
    }

    pub fn solve(file_name: &str) -> u64 {
        let mut hands = Vec::new();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            hands.push(Hand::from_string(line));
        }
        hands.sort_unstable_by(|lhs, rhs| cmp(&lhs, &rhs));
        solve_hands(&hands)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn preprocess_test() {
            assert_eq!("KTTTT", preprocess_hand("KTJJT"));
            assert_eq!("23455", preprocess_hand("2345J"));
            assert_eq!("22333", preprocess_hand("2233J"));
            assert_eq!("AAAAA", preprocess_hand("JJJJJ"));
            assert_eq!("QQ2Q2", preprocess_hand("JQ2Q2"));
        }

        #[test]
        fn solve_test() {
            // here we sort inside preprocess, so this is in arbitrary order
            let mut hands = vec![
                Hand::from_string("32T3K 765"),
                Hand::from_string("KTJJT 220"),
                Hand::from_string("KK677 28"),
                Hand::from_string("T55J5 684"),
                Hand::from_string("QQQJA 483"),
            ];
            hands.sort_unstable_by(|lhs, rhs| cmp(&lhs, &rhs));
            assert_eq!(5905, solve_hands(&hands));
        }
    }
}
