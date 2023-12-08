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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        let result = self.strength().cmp(&other.strength());
        Some(match result {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let lhs_score = get_card_score(self.cards.as_bytes()[i] as char);
                    let rhs_score = get_card_score(other.cards.as_bytes()[i] as char);
                    if lhs_score == rhs_score {
                        continue;
                    }
                    let result = lhs_score.cmp(&rhs_score);
                    return Some(result);
                }
                std::cmp::Ordering::Equal
            }
            _ => result,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
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

    #[test]
    fn hand_cmp_test() {
        assert!(Hand::from_string("32T4K 765") < Hand::from_string("T55J5 684"));
        assert!(Hand::from_string("32T3K 765") < Hand::from_string("T55J5 684"));
        assert!(Hand::from_string("KK677 28") > Hand::from_string("KTJJT 220"));
        assert!(Hand::from_string("KK677 28") < Hand::from_string("QQQJA 483"));
    }
}

fn solve_hands(hands: &Vec<Hand>) -> u64 {
    let mut result: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i as u64 + 1);
        println!("{:?}, {}", hand, result);
    }

    result
}

pub mod part1 {
    use super::*;

    pub fn solve(file_name: &str) -> u64 {
        let mut hands = Vec::new();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            hands.push(Hand::from_string(line));
        }
        hands.sort_unstable();
        solve_hands(&hands)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
