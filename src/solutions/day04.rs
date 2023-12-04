use std::{collections::HashSet, str::FromStr};

use super::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn test_input() -> String {
        String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
    }

    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|line| line.trim().parse::<ScratchCard>().unwrap())
            .map(|card| card.score())
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

struct ScratchCard {
    wins: HashSet<usize>,
    nums: Vec<usize>,
}

impl ScratchCard {
    fn score(&self) -> usize {
        let matches: Vec<usize> = self
            .nums
            .iter()
            .filter_map(|n| {
                if let Some(_) = self.wins.get(n) {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        match matches.len() {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.split(":");
        let mut parts = colon
            .skip(1)
            .next()
            .unwrap()
            .split("|")
            .map(|part| part.trim());
        let wins = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        let nums = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();

        Ok(Self { wins, nums })
    }
}
#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_1(input);
        assert_eq!(ans, "13");
    }

    #[test]
    fn test_part_2() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
