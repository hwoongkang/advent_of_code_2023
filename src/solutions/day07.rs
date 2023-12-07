use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn test_input() -> String {
        String::from(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut game: Vec<(CamelHand, usize)> = input
            .lines()
            .map(|l| {
                let mut words = l.trim().split_whitespace();
                let hand = words.next().unwrap().parse().unwrap();
                let bid = words.next().unwrap().parse().unwrap();
                (hand, bid)
            })
            .collect();
        game.sort_by(|s, o| s.0.cmp(&o.0));
        game.iter()
            .enumerate()
            .map(|(ind, (_, bid))| bid * (ind + 1))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct CamelCard(char);

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);
        let s = order.get(&self.0);
        let o = order.get(&other.0);
        if s.is_none() || o.is_none() {
            panic!("sth wrong");
        }
        s.partial_cmp(&o)
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct CamelHand {
    cards: [CamelCard; 5],
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s = self.get_hand_type();
        let o = other.get_hand_type();
        match s.cmp(&o) {
            Ordering::Equal => {
                for i in 0..5 {
                    let s = self.cards[i];
                    let o = other.cards[i];
                    if s != o {
                        return s.partial_cmp(&o);
                    }
                }
                Some(Ordering::Equal)
            }
            some => return Some(some),
        }
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl CamelHand {
    fn get_hand_type(&self) -> CamelHandType {
        let mut counts: HashMap<CamelCard, usize> = HashMap::new();
        for card in self.cards.iter() {
            if let Some(num) = counts.get_mut(&card) {
                *num += 1;
            } else {
                counts.insert(*card, 1);
            }
        }
        let l = counts.len();
        match l {
            5 => CamelHandType::HighCard,
            4 => CamelHandType::OnePair,
            3 => {
                if *counts.values().max().unwrap() == 3 {
                    CamelHandType::ThreeOfAKind
                } else {
                    CamelHandType::TwoPairs
                }
            }
            2 => {
                if *counts.values().max().unwrap() == 4 {
                    CamelHandType::FourOfAKind
                } else {
                    CamelHandType::FullHouse
                }
            }
            1 => CamelHandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl FromStr for CamelHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let cards: [CamelCard; 5] = [
            CamelCard(chars[0]),
            CamelCard(chars[1]),
            CamelCard(chars[2]),
            CamelCard(chars[3]),
            CamelCard(chars[4]),
        ];
        Ok(Self { cards })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CamelHandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn test_ordering_card() {
        let king = CamelCard('K');
        let jack = CamelCard('J');
        assert!(king > jack);
    }

    #[test]
    fn parse_camel_hand() {
        let hand = "7TQQJ".parse::<CamelHand>();
        assert_eq!(
            hand,
            Ok(CamelHand {
                cards: [
                    CamelCard('7'),
                    CamelCard('T'),
                    CamelCard('Q'),
                    CamelCard('Q'),
                    CamelCard('J')
                ]
            })
        )
    }

    #[test]
    fn test_part_1() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_1(input);
        assert_eq!(ans, "6440");
    }

    #[test]
    fn test_part_2() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
