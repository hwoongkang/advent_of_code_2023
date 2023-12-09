use std::{cell::RefCell, rc::Rc};

use super::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn test_input() -> String {
        String::from(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        )
    }

    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|line| {
                let nums: Vec<i64> = line
                    .trim()
                    .split_whitespace()
                    .map(|w| w.trim().parse().unwrap())
                    .collect();
                nums
            })
            .map(|nums| OasisHistory::from(&nums))
            .map(|oasis| oasis.predict())
            .sum::<i64>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

#[derive(Debug)]
enum OasisHistory {
    Cons(i64, Rc<RefCell<OasisHistory>>),
    Nil,
}

impl OasisHistory {
    fn from(nums: &[i64]) -> Self {
        let diffs: Vec<i64> = nums
            .iter()
            .zip(nums.iter().skip(1))
            .map(|(prev, next)| next - prev)
            .collect();
        if nums
            .iter()
            .filter_map(|&n| if n == 0 { None } else { Some(()) })
            .count()
            == 0
        {
            Self::Nil
        } else {
            Self::Cons(
                *nums.last().unwrap(),
                Rc::new(RefCell::new(Self::from(&diffs))),
            )
        }
    }
    fn predict(&self) -> i64 {
        match self {
            Self::Nil => 0,
            Self::Cons(last_value, child) => last_value + child.borrow().predict(),
        }
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day09::test_input();
        let ans = Day09::solve_part_1(input);
        assert_eq!(ans, "114");
    }

    #[test]
    fn test_part_2() {
        let input = Day09::test_input();
        let ans = Day09::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
